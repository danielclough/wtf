<script>
    $: mode = 'login'
</script>
<section>
    <form method="POST" action="?/login">
        {#if mode == 'register'}
            <label>
                First Name
                <input name="first_name" type="text">
            </label>
    
            <label>
                Last Name
                <input name="last_name" type="text">
            </label>
        {/if}
    
        <label>
            Email
            <input name="email" type="email">
        </label>
        
        <label>
            Password
            <input name="password" type="password">
        </label>
        
        {#if mode == 'register'}
            <label>
                Multi Factor Key (optional)
                <input name="mfa" type="text">
            </label>
            
            <button formaction="?/register">Register</button>
        {:else}
            <button formaction="?/login">Login</button>
        {/if}
    </form>
    <button on:click={()=>mode == 'register' ? mode="login" : mode= "register"}>
        {mode == 'register' ? "Login" : "Register"}
    </button>
</section>

<style>
    form {
        display:flex;
        flex-direction:column;
    }
    section button {
        width: fit-content;
        align-self: center;
    }
    form button {
        width: fit-content;
        align-self: end;
        padding: .25rem;
        margin: .5rem;
    }
</style>