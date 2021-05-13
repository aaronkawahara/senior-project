cd D:\Projects\senior-project\game-dude\scripts
1 | python3 .\bmp_converter.py
rm ..\cross\games\assets\*.bmp
mv .\bmp\rgb332\*.bmp ..\cross\games\assets
$files = (ls ..\cross\games\assets\*.bmp).FullName -join(' ')
python3 .\make_array_from_bmp.py -o ..\cross\games\src\images.rs $files