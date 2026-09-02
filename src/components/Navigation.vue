<script setup lang="ts"></script>

<template>
    <nav>
        <RouterLink :to="{name: 'accueil'}">Accueil</RouterLink>
        <RouterLink :to="{name: 'palettes'}">Palettes</RouterLink>
        <RouterLink :to="{name: 'utilitaires'}">Utilitaires</RouterLink>
    </nav>
</template>