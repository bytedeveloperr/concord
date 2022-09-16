<template>
  <v-row align="center" justify="center" style="height: 99vh">
    <v-col md="6" class="mx-auto">
      <h5 class="text-h5">Connect your wallet</h5>
      <p class="">Please, click any of the buttons below to signin with your wallet</p>

      <v-list>
        <v-list-item border v-for="(item, i) in items" :key="i" :value="item" class="my-3" active-color="primary">
          <template v-slot:prepend>
            <v-avatar size="25" rounded="0">
              <v-img :src="item.logo" :alt="item.title" />
            </v-avatar>
          </template>

          <v-list-item-title v-text="item.title" />
        </v-list-item>
      </v-list>
    </v-col>
  </v-row>
</template>

<script setup>
const items = [
  { logo: "/assets/images/near.svg", title: "Near Wallet" },
  { logo: "/assets/images/sender.svg", title: "Sender Wallet" },
];
</script>
