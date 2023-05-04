<script lang="ts">
	import { onMount } from "svelte";

    let faceapi;
    let detections = [];

    let video;
    let canvas;

    onMount(()=>{
        const s = (sketch: any) => {

            function setup() {
                canvas = sketch.createCanvas(1080, 720); // canvas window
                canvas.id("canvas");
        
                // getting video of user
                video = sketch.createCapture(video);
                video.id("video");
                video.size(width, height);
        
                // making face details
                const faceOptions = {
                    withLandmarks: true,
                    withExpressions: true,
                    withDescriptors: true,
                    minConfidence: 0.5,
                };
        
                //Initialize the model:
                faceapi = ml5.faceApi(video, faceOptions, faceReady);
            }
        
            // on face detection
            function faceReady() {
                faceapi.detect(gotFaces);
            }
        
            // Got faces:
            function gotFaces(error, result) {
                if (error) {
                    console.log(error);
                    return;
                }
        
                detections = result; //Now all the data in this detections:
        
                clear(); //Draw transparent background;:
                drawBoxs(detections); //Draw detection box:
                drawLandmarks(detections); //// Draw all the face points:
        
                faceapi.detect(gotFaces); // Call the function again at here:
            }
        
            function drawBoxs(detections) {
                if (detections.length > 0) {
                    //If at least 1 face is detected:
                    for (f = 0; f < detections.length; f++) {
                        let { _x, _y, _width, _height } = detections[f].alignedRect._box;
                        stroke(44, 169, 225);
                        strokeWeight(1);
                        noFill();
                        rect(_x, _y, _width, _height);
                    }
                }
            }
        
            function drawLandmarks(detections) {
                if (detections.length > 0) {
                    //If at least 1 face is detected:
                    for (f = 0; f < detections.length; f++) {
                        let points = detections[f].landmarks.positions;
                        for (let i = 0; i < points.length; i++) {
                            stroke(47, 255, 0); // points color
                            strokeWeight(5); // points weight
                            point(points[i]._x, points[i]._y);
                        }
                    }
                }
            }
        }  
    })

</script>

<svelte:head>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/p5.js/1.1.9/p5.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/p5.js/1.1.9/addons/p5.sound.js"></script>
    <script src="https://unpkg.com/ml5@0.6.1/dist/ml5.min.js" type="text/javascript"></script>
</svelte:head>

<style>
    #canvas {
    transform: translate(-50%, -50%);
    z-index: 1;
    }

    #video {
    transform: translate(-50%, -50%);
    z-index: 0;
    border: 3px #fff solid;
    border-radius: 10px;
    }
</style>