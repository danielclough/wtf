let swipe_left = `
<div style="display:relative">
    <div class="semi-opaque" > 👈 Swipe Left </div>
</div>
`

let styles = `
<style>
    small {
        vertical-align: unset !important;
    }
</style>
`

export default [
    {
        html: `
            <h1>
                <small>
                    I will read each prompt
                </small>
                <br>

                out loud.
                <br>

            </h1>
            ${swipe_left}
        `,
    },
    {
        html: `
            <h1>
                I will ask questions
            </h1>
        `,
    },
    {
        html: `
        `,
        btns: [
            {
                details: ["I understand that this recording may be published online."],
                text: ["Yes", "No"],
            }
        ],
        opt: swipe_left
    },
    {
        html: `
        `,
        btns: [
            {
                details: ["I want to be contacted if this video is published."],
                text: ["Yes", "No"],
            }
        ],
        opt: `
            <form>
                <label for="first_name">
                    First Name
                    <input type="first_name" id="first_name" name="first_name" >
                </label>
                <br>
                <label for="email">
                    Email
                    <input type="email" id="email" name="email" >
                </label>
            </form>
        `,
    },
    // {
    //     html: ``,
    //     btns: [
    //         {
    //             details: ["Do you want your face and/or voice to be anonymized before uploading?"],
    //             text: ["Anonymize my face", "Anonymize both"],
    //         },
    //         {
    //             details: ["Do you require that the the anonymization process be non-reversible by standard software?"],
    //             text: ["Standard", "Non-Reversible"],
    //         }
    //     ],
    // },
    {
        html: `
        Face the camera... 
        <h1>
            clap 👏
        </h1>
        then press Start!
        `,
    }

]