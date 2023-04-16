#start one more than the last availible file
i=39
for image in *.jpg; do
    convert ${image} -resize 750x250^ -gravity center -extent 750x250 ../jpg/background${i}.jpg
    rm ${image}
    (( i=i+1 )) 
done;