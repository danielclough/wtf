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

export default {
version: `Verbal v1`,
questions: [
    {
        html: `
        <h1>
            <small>
                Read each prompt
            </small>
            <br>
            out loud.
        </h1>
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
                    My Name Is (spell if tricky)
                    <input hidden type="first_name" id="first_name" name="first_name" >
                </label>
                <br>
                <label for="email">
                    My Email Is (spell if tricky)
                    <input hidden type="email" id="email" name="email" >
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
        Let's Go!
        `,
    }
]}