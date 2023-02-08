# script to build the docker file

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd $SCRIPT_DIR

if ! test -f sgx_rpm_local_repo.tgz; then
    wget https://download.01.org/intel-sgx/sgx-dcap/1.15/linux/distro/Anolis86/sgx_rpm_local_repo.tgz
fi


tar zxvf sgx_rpm_local_repo.tgz

docker build -t anolis86-tdx-dragonball --network host .
