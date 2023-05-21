export async function start(mediaRecorder: MediaRecorder) {
    const audio: Element|null = document.querySelector('#audio');
    if (audio !== null) {

        let stream = await navigator.mediaDevices.getUserMedia({
            audio: true, video: false
        })

        mediaRecorder = new MediaRecorder(stream);
        mediaRecorder.start();

        let chunks: any = [];
        mediaRecorder.ondataavailable = (e)=>{
            chunks.push(e.data);
        }
        //function to catch error
        mediaRecorder.onerror = (e)=>{
            alert(e.error);
        }
        mediaRecorder.onstop = ()=>{
            let blob = new Blob(chunks);
            //create url for audio
            let url = URL.createObjectURL(blob);
            // clone and replace audio element
            const clone = audio.cloneNode()
            audio.replaceWith(clone)
            //pass url into cloned audio
            clone.src = url;
            console.log(stream.id, url);
            // stop recording
            stream.getTracks().forEach( track => track.stop() );
            
            // download file
            const link = document.createElement("a");
            link.href = url;
            //  grab uuid from url
            link.download = `${url.split('/')[3]}.ogx`;
            link.click();
        }

    }
    return mediaRecorder
}

export async function stop(mediaRecorder: MediaRecorder) {
    mediaRecorder.stop()
    setTimeout(()=>{}, 3000)
    
    return mediaRecorder
}
