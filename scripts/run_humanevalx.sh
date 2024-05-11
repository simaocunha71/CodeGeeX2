#!/bin/bash
# This script is used to generate solutions of HumanEval-X.

# Examples (MODE=(gen, eval, both)):
# MODE=gen bash ./scripts/run_humanevalx.sh

MODE=eval

SCRIPT_PATH=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
MAIN_DIR=$(dirname "$SCRIPT_DIR")

# Environment settings
HOSTLIST=$SCRIPT_DIR/hostlist
WORLD_SIZE=1
DATASET=humanevalx
GENERATION_MODE=completion
MODEL_NAME=codegeex2-6b
MODEL_PATH=/pathto/codegeex2-6b/
N_CPU_WORKERS=16
TIMEOUT=5

# Generation settings
## pass@1 greedy
NUM_SAMPLES=1
MICRO_BSZ=1
TEMP=1.0
TOPK=1
TOPP=1.0
MAX_LENGTH=1024
SEED=42
GREEDY=1

for l in rust;

#for l in python java js cpp go rust;
do
    LANGUAGE=$l
    DATA_DIR=$MAIN_DIR/benchmark/$DATASET/
    DATA_PATH=$DATA_DIR/$DATASET\_$LANGUAGE.jsonl.gz
    OUTPUT_PATH=$MAIN_DIR/output/$DATASET/$LANGUAGE
    TODAY=$(date +%y%m%d)
    CHANNEL_PORT=$((RANDOM + 5000))
    MASTER_PORT=$((RANDOM + 8000))
    JOB_ID=$MODEL_NAME-$LANGUAGE-greedy$GREEDY-ns$NUM_SAMPLES-t$TEMP-topp$TOPP-seed$SEED
    mkdir -p "$OUTPUT_PATH/$JOB_ID"

    # Evaluation settings
    EVAL_INPUT_PATH=$OUTPUT_PATH/$JOB_ID
    EVAL_OUTPUT_PATH=$OUTPUT_PATH/$JOB_ID

    eval_func() {
        echo "Evaluating......"

        if [ $LANGUAGE = rust ]; then
            TIMEOUT=300
            echo "Setting timeout to $TIMEOUT for Rust"
        fi
        RUN_CMD="python3 \
            $MAIN_DIR/evaluation/evaluation.py \
            --input_path "samples_llama-2-7b.Q2_K_humaneval_rust.jsonl" \
            --output_path $EVAL_OUTPUT_PATH \
            --log-path $OUTPUT_PATH/$JOB_ID/$TODAY-evaluation.log \
            --model_name $MODEL_NAME \
            --language_type $LANGUAGE \
            --dataset_type $DATASET \
            --generation_mode $GENERATION_MODE \
            --n_workers $N_CPU_WORKERS \
            --tmp_dir $MAIN_DIR/benchmark/$DATASET/$LANGUAGE \
            --problem_file $DATA_PATH \
            --timeout $TIMEOUT"

        # Inspecting results
        INSPECT_CMD="python3 \
            $MAIN_DIR/evaluation/inspect_jsonl.py \
            --data_path $EVAL_OUTPUT_PATH/result_samples_llama-2-7b.Q2_K_humaneval_rust.jsonl \
            --log-path $OUTPUT_PATH/$JOB_ID/$TODAY-inspect.txt"

        eval "$RUN_CMD && $INSPECT_CMD"
    }

    case $MODE in
    "eval")
        eval_func
        ;;
    *)
        echo "Unsupported mode (eval): $MODE"
        exit 1
        ;;
    esac
done
