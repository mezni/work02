<template>
  <img class="logo" src="../assets/resto.jpeg" />
  <h1>Sign Up</h1>
  <div class="register">
    <input type="text" v-model="name" id="name" placeholder="Enter Name" />
    <input type="email" v-model="email" id="email" placeholder="Enter Email" />
    <input
      type="password"
      v-model="password"
      id="password"
      placeholder="Enter Password"
    />
    <button v-on:click="signUp">Sign Up</button>
  </div>
</template>

<script>
export default {
  name: "SignUp",
  data() {
    return {
      name: "",
      email: "",
      password: "",
    };
  },
  methods: {
    signUp() {
      console.warn("signup");
    },
  },
};
</script>

<style>
.logo {
  width: 100px;
}
.register input {
  width: 300px;
  height: 40px;
  padding-left: 20px;
  display: block;
  margin-bottom: 30px;
  margin-right: auto;
  margin-left: auto;
  border: 1px solid skyblue;
}

.register button {
  width: 320px;
  height: 40px;
  border: 1px solid skyblue;
  background: skyblue;
  color: white;
  cursor: pointer;
}
</style>