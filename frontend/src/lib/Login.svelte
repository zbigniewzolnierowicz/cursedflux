<script lang="ts">
    import { postJson } from "../utils/api";

    const EMAIL_NAME = "email"
    const PASSWORD_NAME = "password"

    async function submitForm(event: SubmitEvent) {
        event.preventDefault()
        const formData = new FormData(event.target as HTMLFormElement)
        const email = formData.get(EMAIL_NAME)
        if (email instanceof File) return
        const password = formData.get(PASSWORD_NAME)
        if (password instanceof File) return
        const data = { email, password };

        const response = await postJson("/api/user/login", data);

        console.log(response);
    }
</script>

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
</form>

<style>
    form.login-form {
        display: flex;
        flex-direction: column;
    }
</style>