@echo off
del f.exe flat.exe pe64demo.exe
echo flat x64
fasm.exe stage0.asm f.exe -s f.fas
f.exe stage1.asm flat.exe -s flat.fas
flat.exe pe64demo.asm pe64demo.exe
pe64demo.exe
