set -e

# echo "Converting NOAA's GLOBE data set into a usable format"
# mkdir -p elevation_data/derived
#
# echo "Transforming tiles ..."
# mkdir -p elevation_data/derived/transformed_tiles
#
# for i in {a..p}; do
#   echo "Transforming tile $i"
#   raw-bytes-math 500 < "elevation_data/raw/${i}10g" > "elevation_data/derived/transformed_tiles/$i"
# done

# echo "Downsampling tiles ..."
# mkdir -p elevation_data/derived/transformed_tiles_small
#
# for i in {a..d}; do
#   echo "Downsampling tile $i"
#   convert -monitor -depth 16 -endian lsb -size 10800x4800 gray:elevation_data/derived/transformed_tiles/$i -resize 2700x1200\! elevation_data/derived/transformed_tiles_small/$i
# done
#
# for i in {e..l}; do
#   echo "Downsampling tile $i"
#   convert -monitor -depth 16 -endian lsb -size 10800x6000 gray:elevation_data/derived/transformed_tiles/$i -resize 2700x1500\! elevation_data/derived/transformed_tiles_small/$i
# done
#
# for i in {m..p}; do
#   echo "Downsampling tile $i"
#   convert -monitor -depth 16 -endian lsb -size 10800x4800 gray:elevation_data/derived/transformed_tiles/$i -resize 2700x1200\! elevation_data/derived/transformed_tiles_small/$i
# done

# echo "Putting the tiles together ..."
# mkdir -p elevation_data/derived/transformed_rows
#
# montage -monitor -depth 16 -size 2700x1200 gray:elevation_data/derived/transformed_tiles_small/[a-d] -tile 4x1 -geometry +0+0 elevation_data/derived/transformed_rows/1
# montage -monitor -depth 16 -size 2700x1500 gray:elevation_data/derived/transformed_tiles_small/[e-l] -tile 4x2 -geometry +0+0 elevation_data/derived/transformed_rows/2
# montage -monitor -depth 16 -size 2700x1200 gray:elevation_data/derived/transformed_tiles_small/[m-p] -tile 4x1 -geometry +0+0 elevation_data/derived/transformed_rows/3

# mkdir -p elevation_data/derived/transformed_full
# montage -monitor -depth 16 \
#   -size 10800x1200 gray:elevation_data/derived/transformed_rows/1 \
#   -size 10800x3000 gray:elevation_data/derived/transformed_rows/2 \
#   -size 10800x1200 gray:elevation_data/derived/transformed_rows/3 \
#   -tile 1x3 -geometry +0+0 elevation_data/derived/transformed_full/10800x5400

mkdir -p elevation_data/derived/transformed_final_tiles
convert -monitor -depth 16 -size 10800x5400 gray:elevation_data/derived/transformed_full/10800x5400 -crop 450x450 elevation_data/derived/transformed_final_tiles/%d
