set -e

for i in {a..d}; do
  echo "CONVERTING $i"
  convert -monitor -depth 16 -endian lsb -size 10800x4800 gray:elevation_data/derived/transformed/tiles/$i -resize 2700x1200\! elevation_data/derived/transformed/tiles/$i-small
done

for i in {e..l}; do
  echo "CONVERTING $i"
  convert -monitor -depth 16 -endian lsb -size 10800x6000 gray:elevation_data/derived/transformed/tiles/$i -resize 2700x1500\! elevation_data/derived/transformed/tiles/$i-small
done

for i in {m..p}; do
  echo "CONVERTING $i"
  convert -monitor -depth 16 -endian lsb -size 10800x4800 gray:elevation_data/derived/transformed/tiles/$i -resize 2700x1200\! elevation_data/derived/transformed/tiles/$i-small
done

montage -monitor -depth 16 -size 2700x1200 gray:elevation_data/derived/transformed/tiles/[a-d]-small -tile 4x1 -geometry +0+0 elevation_data/derived/transformed/tiles/row-1
montage -monitor -depth 16 -size 2700x1500 gray:elevation_data/derived/transformed/tiles/[e-l]-small -tile 4x2 -geometry +0+0 elevation_data/derived/transformed/tiles/row-2
montage -monitor -depth 16 -size 2700x1200 gray:elevation_data/derived/transformed/tiles/[m-p]-small -tile 4x1 -geometry +0+0 elevation_data/derived/transformed/tiles/row-3

montage -monitor -depth 16 \
  -size 10800x1200 gray:elevation_data/derived/transformed/tiles/row-1 \
  -size 10800x3000 gray:elevation_data/derived/transformed/tiles/row-2 \
  -size 10800x1200 gray:elevation_data/derived/transformed/tiles/row-3 \
  -tile 1x3 -geometry +0+0 elevation_data/derived/transformed/full-10800x5400
