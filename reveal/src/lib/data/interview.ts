let swipe_up = `
<div style="display:relative">
    <div class="semi-opaque left-center" > 👈 Swipe Up on the Side. </div>
</div>
`
export default {
    version: `Verbal v1`,
    questions: [
        {
            subject: `Government`,
            prompts: [
                `
                    <h1>Do I trust my government?</h1>
                    ${swipe_up}
                `,
                `
                    <h1>
                        <small>
                            Is 
                        </small>
                        Mass Surveillance
                        <small>
                            by the government
                        </small>
                        Justified?
                    </h1>
                `,
                `
                    <h1>
                        <small>
                            Is there an
                        </small>
                        institution
                        <small>
                            I trust more than my
                        </small>
                        government?
                    </h1>
                    <p class="semi-opaque">
                        👆
                        <small>
                            End of section.
                        </small>
                        Swipe Left 👉
                    </p>
                `,
            ]
        },
        {
            subject: `Consciousness`,
            prompts: [
                `
                    <h1>
                        <small>
                            Should people be free to 
                        </small>
                        Alter Consciousness
                        <small>
                            with Drugs?
                        </small>
                    </h1>
                    ${swipe_up}
                `,
                `
                    <h1>
                        <small>
                            Is it
                        </small>
                        Possible
                        <small>
                            for Computers to be
                        </small>
                        Sentient?
                    </h1>
                `,
                `
                    <h1>
                        <small>
                            What is
                        </small>
                        Free Will?
                        <small>
                            Does it
                        </small>
                        Exist?
                    </h1>
                    <p class="semi-opaque">
                        👆
                        <small>
                            End of section.
                        </small>
                        Swipe Left 👉
                    </p>
                `,
            ]
        },
        {
            subject: `Is it Real?`,
            prompts: [
                `
                    <h1>Standard Cosmology</h1>
                    <p>
                        Black Holes, Dark Matter, Dark Energy, etc.
                    </p>
                `,
                `
                    <h1>Evolution by Natural Selection?</h1>
                    <p><strong>Life</strong> evolves <strong>chaotically</strong> by <strong>random mutation</strong>.</p>
                `,
                `
                    <h1>A Mystical Story?</h1>
                    <p>Karma, Miracles, Dualism, etc.</p>
                `,
                `
                    <h1>
                        Covid 19?
                    </h1>
                    <p>Germ Theory</p>
                `,
                `
                    <h1>Quantum Mechanics?</h1>
                    <p>Many Worlds?</p>
                    <p>Wave Collapse?</p>
                    <p>Hidden Variables?</p>
                `,
                `
                    <h1><small>We live in a </small> Simulation / Hologram?</h1>
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
            subject: `Who / What should be a Legal Person?`,
            prompts: [
                `
                    ${swipe_up}
                    <small>
                        (Watch this
                        <a href="https://www.youtube.com/watch?v=GxM9BZeRrUI&pp=ygUKcGVyc29uaG9vZA%3D%3D">PBS video</a>
                        to learn about Personhood - 16 sec. clip below)
                    </small>
                    <video controls poster="/video/personhood.jpeg" src="/video/personhood.mp4" type="video/mp4" />
                `,
                `<h1>Superman?</h1>`,
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
                    <p> Like, Subscribe, Follow </p>
                    <img style="width:25%;border:none;" src="/images/YOUTUBE-LOGO.png" alt="YouTube Logo" />
                    <img style="width:25%;border:none;" src="https://chart.googleapis.com/chart?chs=200x200&cht=qr&chl=https://www.youtube.com/channel/UC3uFKCe5MYhI8-iEA9ez2OA" alt="video link" />
                    <img style="width:25%;border:none;" src="/images/YOUTUBE-LOGO.png" alt="YouTube Logo" />
                    <br>
                `
            ]
        }
    ]
}