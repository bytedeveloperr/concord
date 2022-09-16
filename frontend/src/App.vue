<template>
  <v-app>
    <topbar v-if="!hiddenBarPages.includes($route.name)" />

    <v-main>
      <v-container>
        <router-view />
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup>
import Topbar from "./components/Topbar.vue";

const hiddenBarPages = ["connect"];
</script>
