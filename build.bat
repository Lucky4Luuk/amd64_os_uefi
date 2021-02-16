rem This script will call the right build commands from the right working directories
rem then run the kernel in Qemu with the right parameters

echo Beginning build process

pushd "kernel"
echo Building the kernel...
cargo kbuild
echo Constructing bootable image...
cargo run --package boot
popd
echo Starting QEMU [bios]...
qemu-system-x86_64 -drive format=raw,file=target/x86_64-amd64_os_uefi/debug/bootimage-bios-kernel.img
