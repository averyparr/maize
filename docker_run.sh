DOCKER_IMG=nvidia/cuda:13.0.1-devel-rockylinux9

EXEC_PATH=$1

echo "Aliasing '$EXEC_PATH' to 'docker run -v \$(pwd):/work -w /work $DOCKER_IMG $EXEC_PATH'"

eval "$EXEC_PATH() {
    docker run -v \$(pwd):/work -w /work $DOCKER_IMG $EXEC_PATH \"\$@\"
}"
