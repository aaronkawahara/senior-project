cd D:\Projects\senior-project\game-dude\scripts
python3 .\bmp_converter.py
1
mv .\bmp\rgb332\*.bmp D:\Projects\senior-project\game-dude\games\assets
cd D:\Projects\senior-project\game-dude
$file_names = Get-ChildItem -Path D:\Projects\senior-project\game-dude\games\assets\*.bmp -Name
foreach ($fname in $file_names) {
    python3 .\scripts\make_array_from_bmp.py -i .\scripts\bmp\$fname -o .\games\src\images.rs
}


