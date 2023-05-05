#!/bin/bash

echo -e "Insert Zoom mSD:\n
Enter to continue, n cancel"
read zoom
zoom_base_dir='/media/daniel/0403-0201'
if [[ $zoom != "n" ]];then
    mv ${zoom_base_dir}/STEREO/FOLDER01/* wav_files/
    sudo umount $zoom_base_dir
fi


echo -e "Insert Sony SD:\n
Enter to continue, n cancel"
read sony
sony_base_dir='/media/daniel/disk'
if [[ $sony != "n" ]];then
    mv ${sony_base_dir}/PRIVATE/AVCHD/BDMV/STREAM/* video_files/
    thunar  video_files/
    sudo umount ${sony_base_dir}
fi


echo -e "Insert phone SD:\n
Enter to continue, n cancel"
read phone
phone_base_dir='thunar mtp://SAMSUNG_SAMSUNG_Android_RFCN103513Y/Internal%20storage/Documents'
if [[ $phone != "n" ]];then
    thunar ${phone_base_dir}
    thunar phone_files/

    echo -e "Copy Files\n
    Press Enter when done"
    read done
    sudo umount ${phone_base_dir}
fi