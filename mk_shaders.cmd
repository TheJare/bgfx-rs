SET _MYDIR=%~dp0\..
SET _BGFXSRC=%MYDIR%\bgfx-sys\bgfx\src
SET _EXAMPLESRC=%MYDIR%\bgfx-sys\bgfx\examples
SET _SHADERC=%MYDIR%\bgfx-sys\bgfx\tools\bin\windows\shaderc.exe

SET _VSOGL=%_SHADERC% --type vertex --platform linux -p 120 -i %_BGFXSRC%
SET _FSOGL=%_SHADERC% --type fragment --platform linux -p 120 -i %_BGFXSRC%

mkdir %_MYDIR%\examples\assets


mkdir %_MYDIR%\examples\assets\02-metaballs
SET _SRC=%_EXAMPLESRC%\02-metaballs
SET _DEST=%_MYDIR%\examples\assets\02-metaballs
mkdir %_MYDIR%\examples\assets\02-metaballs\OpenGL
%_VSOGL% -f %_SRC%\vs_metaballs.sc -o %_DEST%\OpenGL\vs_metaballs.bin --varyingdef %_SRC\varying.def.sc
%_FSOGL% -f %_SRC%\fs_metaballs.sc -o %_DEST%\OpenGL\fs_metaballs.bin --varyingdef %_SRC\varying.def.sc
