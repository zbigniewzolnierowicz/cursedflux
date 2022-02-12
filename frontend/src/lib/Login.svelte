<script lang="ts">
    import { api } from "../utils/api";

    const EMAIL_NAME = "email"
    const PASSWORD_NAME = "password"
    let resultJwt: string | null = null;
    let userData: string | null = null;

    async function submitForm(event: SubmitEvent) {
        event.preventDefault()
        const formData = new FormData(event.target as HTMLFormElement)
        const email = formData.get(EMAIL_NAME)
        if (email instanceof File) return
        const password = formData.get(PASSWORD_NAME)
        if (password instanceof File) return
        const data = { email, password };

        const response = await api.post("/api/user/login", JSON.stringify(data), { headers: { "Content-Type": "application/json" }, withCredentials: true });

        resultJwt = response.data;
    }

    async function getUserData() {
        const response = await api.get("/api/user", { withCredentials: true });
        userData = response.data;
    }
</script>

<button on:click={getUserData}>
    Get user data
</button>
{#if userData}
    <div>
        {userData}
    </div>
{/if}

<form on:submit={submitForm} class="login-form" id="login-form">
    <label for="login-email">
        Email address
    </label>
    <input type="text" name={EMAIL_NAME} id="login-email" />

    <label for="login-password">
        Password
    </label>
    <input type="password" name={PASSWORD_NAME} id="login-password" />
    <button type="submit">Submit</button>
    {#if resultJwt !== null}
    <div>
        {resultJwt}
    </div>
    {/if}
</form>

<style>
    form.login-form {
        display: flex;
        flex-direction: column;
    }
</style>