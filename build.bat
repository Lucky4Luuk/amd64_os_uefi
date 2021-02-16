@ECHO OFF

rem This script will call the right build commands from the right working directories
rem then run the kernel in Qemu with the right parameters

echo Beginning build process

pushd "kernel"
echo Building the kernel...
echo > cargo kbuild
cargo kbuild
IF ERRORLEVEL 1 GOTO ERROR
echo Constructing bootable image...
echo > cargo run --package boot
cargo run --package boot
IF ERRORLEVEL 1 GOTO ERROR
popd
echo Starting QEMU [bios]...
qemu-system-x86_64 -drive format=raw,file=target/x86_64-amd64_os_uefi/debug/bootimage-bios-kernel.img -debugcon stdio
exit

:ERROR
echo Code has errors, building stopped
exit
