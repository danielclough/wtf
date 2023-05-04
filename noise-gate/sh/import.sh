#!/bin/bash

echo -e "Insert Zoom mSD:\n
Enter to continue, n cancel"
read zoom
zoom_base_dir='/media/daniel/0403-0201'
if [[ $zoom != "n" ]];then
    mv ${zoom_base_dir}/STEREO/FOLDER01/* /home/daniel/git/wtf/noise-gate/wav_files/
    sleep 1
    sudo umount $zoom_base_dir
fi


echo -e "Insert Sony SD:\n
<Enter any Key>"
read sony
sony_base_dir='/media/daniel/disk'
if [[ $sony != "n" ]];then
    mv ${sony_base_dir}/PRIVATE/AVCHD/BDMV/STREAM/* /home/daniel/git/wtf/noise-gate/video_files/
    sleep 1
    sudo umount ${sony_base_dir}
fi


echo -e "Insert phone SD:\n
<Enter any Key>"
read phone
phone_base_dir='/run/user/1000/gvfs/'
if [[ $phone != "n" ]];then
    mv "${phone_base_dir}/mtp:host=SAMSUNG_SAMSUNG_Android_RFCN103513Y/Internal storage/Recordings/*" /home/daniel/git/wtf/noise-gate/phone_files/
    sleep 1
    sudo umount ${phone_base_dir}
fi