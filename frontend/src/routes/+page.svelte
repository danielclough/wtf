<script>
    let links = [
        ["/survey/ontology","What Exists?"],
        ["/survey/epistemology","How Do We Know?"],
        ["/survey/ism","*-ism Schism Game"],
    ]
</script>

<section>
    <img src="/images/wtf.svg" alt="WTF? Logo" />

    <h1>
        Seriously Y'all. WTF?
    </h1>

    <h2>
        Tell me what you think!
    </h2>
    {#each links as link}
    <a href={link[0]}> {link[1]} </a>
    {/each}
</section>


<style>
    img {
        width: 50%;
        margin: 0 auto;
    }
</style>