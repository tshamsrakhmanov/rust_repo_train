#!/bin/bash
echo "Check for ImageMagick software..."
if command -v magick &>/dev/null; then

  echo "magick EXISTS"
  echo "Run convertation"

  for file in *.ppm; do

    if [[ -f "$file" ]]; then
      new_file="${file%.ppm}.png"

      echo " Converting $file to $new_file..."
      magick "$file" "$new_file"

      if [[ -f "$new_file" && $(stat --format=%s "$new_file") -gt 0 ]]; then
        rm "$file"
        echo " Original file $file deleted."
      else
        echo " Error converting or creating PNG file. Original file not deleted."
      fi

    else
      echo " No files found with '.ppm' extention"
    fi

  done
else
  echo "magick is NOT installed"
fi
