<script lang="ts">
    const EMAIL_NAME = "email"
    const PASSWORD_NAME = "password"

    async function submitForm(event: SubmitEvent) {
        event.preventDefault()
        const formData = new FormData(event.target as HTMLFormElement)
        const email = formData.get(EMAIL_NAME)
        if (email instanceof File) return
        const password = formData.get(PASSWORD_NAME)
        if (password instanceof File) return
        const response = await fetch("http://localhost:8080/api/user/login", {
            body: JSON.stringify({
                email,
                password
            }),
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            mode: 'cors'
        })

        console.log(await response.json())
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