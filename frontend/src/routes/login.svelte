<script>
    import Swal from "sweetalert2";
    import { push } from 'svelte-spa-router';

    function navigateTo(path) {
        push('/' + path);
    }

    let formData = $state({
        email: "",
        password: ""
    });

    async function submit() {
        const response = await fetch(`https://snakesystem-web-api-tdam.shuttle.app/api/v1/auth/login`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Access-Control-Allow-Origin": "*",
            },
            body: JSON.stringify(formData),
        });
        const result = await response.json();
        if (response.status == 200) {
            await Swal.fire({
                icon: "success",
                title: result.message,
                text: "Login Success",
            })
            await navigateTo("home");
        } else {
            await Swal.fire({
                icon: "error",
                title: result.message,
                text: result.error,
            })
        }
        console.log({result, response});
    }
</script>

<section>
    <div class="container-fluid">
      <div class="row d-flex justify-content-center align-items-center h-100">
        <div class="col-md-9 col-lg-6 col-xl-5">
          <img src="/img/image-login.png" class="img-fluid" alt="">
        </div>
        <div class="col-md-8 col-lg-6 col-xl-4 offset-xl-1">
          <h2 class="fw-bold mb-2 ">UBS Trade Dashboard</h2>
          <p class="mb-4">Please login to your account</p>
          <form onsubmit={(e) => {
            e.preventDefault();
            submit();
          }}>
            <!-- Email input -->
            <div data-mdb-input-init class="form-outline mb-4">
                <label class="form-label" for="email">Email address</label>
                <input type="email" id="email" class="form-control form-control-md" bind:value={formData.email}
                    placeholder="Enter a valid email address" />
            </div>
  
            <!-- Password input -->
            <div data-mdb-input-init class="form-outline mb-3">
                <label class="form-label" for="password">Password</label>
                <input type="password" id="password" class="form-control form-control-md" bind:value={formData.password}
                    placeholder="Enter password" />
            </div>
  
            <div class="d-flex justify-content-between align-items-center">
              <!-- Checkbox -->
              <div class="form-check mb-0">
                <input class="form-check-input me-2" type="checkbox" value="" id="remember" />
                <label class="form-check-label" for="remember">
                  Remember me
                </label>
              </div>
              <a href="#!" class="text-body">Forgot password?</a>
            </div>
  
            <div class="text-center text-lg-start mt-4 pt-2">
              <button  type="submit" class="btn btn-primary w-100">Login</button>
              <p class="small fw-bold mt-2 pt-1 mb-0">Don't have an account? <a href="#!"
                  class="link-danger">Register</a></p>
            </div>
  
          </form>
        </div>
      </div>
    </div>
    <div
      class="d-flex flex-column flex-md-row text-center text-md-start justify-content-between py-4 px-4 px-xl-5 bg-primary copyright">
      <!-- Copyright -->
      <div class="text-white mb-3 mb-md-0">
        Copyright © 2020. All rights reserved.
      </div>
      <!-- Copyright -->
    </div>
</section>

<style scoped>
    section {
        max-height: 100vh;
        overflow-y: auto;
    }

    .container-fluid {
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        position: absolute;
    }

    .copyright {
        position: absolute;
        bottom: 0;
        width: 100%;
    }
</style>