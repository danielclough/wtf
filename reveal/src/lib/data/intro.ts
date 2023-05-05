export default [
    {
        text: `
I am interviewing people to analyze speech and opinion patterns.
        `,
        btns: [
            {
                details: ["Do you have any questions?"],
                text: ["Yes", "No"],
            }
        ],
        html: ""
    },
    {
        text: `

        `,
        btns: [
            {
                details: ["Do you understand that this recording may be published online?"],
                text: ["Yes", "No"],
            }
        ],
        html: ""
    },
    {
        text: `

        `,
        btns: [
            {
                details: ["Do you want to be contacted if this video is published?"],
                text: ["Yes", "No"],
            }
        ],
        html: `
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
        `
    },
    {
        text: `
            Read each question out loud before answering.
        `,
    },
    // {
    //     text: `

    //     `,
    //     btns: [
    //         {
    //             details: ["Do you want your face and/or voice to be anonymized before uploading?"],
    //             text: ["Anonymize my face", "Anonymize both"],
    //             html: ""
    //         },
    //         {
    //             details: ["Do you require that the the anonymization process be non-reversible by standard software?"],
    //             text: ["Standard", "Non-Reversible"],
    //             html: ""
    //         }
    //     ]
    // },
    {
        text: `
            Ask questions at any time.
        `,
    }

]