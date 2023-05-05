export default {
    version: `Verbal v1`,
    questions: [
        {
            subject: `Government`,
            prompts: [
                `
                    <h1>Do I trust my government?</h1>
                    <p class="semi-opaque">👆 Swipe Down 👆</p>
                `,
                `
                    <h1>
                        <small>
                            Is there an</small> institution <small>I trust more than my
                        </small>
                        government?
                    </h1>

                    <p class="semi-opaque">
                        👆
                        <small>
                            End of section.
                        </small>
                        Swipe Right 👉
                    </p>
                `
            ]
        },
        {
            subject: `Is it Real?`,
            prompts: [
                `
                    <h1>Standard Cosmology</h1>
                    <p>
                        Dark Matter, Dark Energy, etc.
                    </p>
                `,
                `
                    <h1>Evolution by Natural Selection?</h1>
                    <p><strong>Life</strong> evolves <strong>chaotically</strong> by <strong>random mutation</strong>.</p>
                `,
                `
                    <h1>A Religious Story?</h1>
                    <p>Karma, Miracles, Resurrection, etc.</p>
                `,
                `<h1>Covid 19?</h1>`,
                `
                    <h1>Quantum Mechanics?</h1>
                    <p>Many Worlds?</p>
                    <p>Wave Collapse?</p>
                    <p>Hidden Variables?</p>
                `,
            ],
        },
        {
            subject: `How do I feel about killing?`,
            prompts: [
                `<h1>Mosquitos?</h1>`,
                `<h1>Stray Dogs?</h1>`,
                `<h1>Hunting for food?</h1>`,
                `<h1>Hunting for pleasure?</h1>`,
                `<h1>Hunting Endangered Species?</h1>`,
                `<h1>Suicide / Euthanasia?</h1>`
            ]
        },
        {
            subject: `Who / What is a Person?`,
            prompts: [
                `
                <img style="position:absolute; top:-3rem;right:0;" src="https://chart.googleapis.com/chart?chs=120x120&cht=qr&chl=https://www.youtube.com/watch?v=GxM9BZeRrUI&pp=ygUKcGVyc29uaG9vZA%3D%3D" alt="video link" />
                <small>
                    (Watch this
                    <a href="https://www.youtube.com/watch?v=GxM9BZeRrUI&pp=ygUKcGVyc29uaG9vZA%3D%3D">PBS video</a>
                    to learn about Personhood - 15 sec.)
                </small>
                <video controls poster="/video/personhood.jpeg" src="/video/personhood.mp4" type="video/mp4" />
                `,
                `<h1>Corporations?</h1>`,
                `<h1>AI?</h1>`,
                `<h1>Intelligent Animals?</h1>`,
                `<h1>Rivers?</h1>`,
                `<h1>Terrorists?</h1>`,
                `<h1>Zygote / Fetus?</h1>
                <img src="https://i0.wp.com/pocketdentistry.com/wp-content/uploads/2017/09/image00209.jpeg?w=960" alt="Stages of Human Development" />`
            ]
        },
        {
            subject: `Thank You for Participating`,
            prompts: [
                `
                    <img style="width:25%;border:none;" src="/images/YOUTUBE-LOGO.png" alt="YouTube Logo" />
                    <img style="width:25%;border:none;" src="https://chart.googleapis.com/chart?chs=200x200&cht=qr&chl=https://www.youtube.com/channel/UC3uFKCe5MYhI8-iEA9ez2OA" alt="video link" />
                    <img style="width:25%;border:none;" src="/images/YOUTUBE-LOGO.png" alt="YouTube Logo" />
                    <br>
                    <a data-sveltekit-reload href="/"><button>Done!</button></a>
                `
            ]
        }
    ]
}