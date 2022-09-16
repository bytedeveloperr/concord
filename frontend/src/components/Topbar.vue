<template>
  <v-sheet border>
    <v-container class="pa-0 fill-height">
      <v-app-bar flat density="compact">
        <v-app-bar-nav-icon class="d-block d-md-none"></v-app-bar-nav-icon>

        <v-app-bar-title class="mr-4" style="font-size: 16px">Concord Finance</v-app-bar-title>

        <v-btn link to="/overview" density="compact" class="px-2">Overview</v-btn>
        <v-btn link to="/collectives" density="compact" class="px-2">Collectives</v-btn>

        <template v-slot:append>
          <v-btn link density="compact" prepend-icon="mdi-account-circle-outline">bytedeveloper.testnet</v-btn>
        </template>
      </v-app-bar>
    </v-container>
  </v-sheet>
</template>
