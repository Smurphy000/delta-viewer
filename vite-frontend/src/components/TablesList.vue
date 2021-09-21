<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'

// let data: unknown = reactive({})
const data = ref<Promise<unknown> | null>(null)
const fetchTables = async () => {
    fetch('http://localhost:3000/api/tables', {
        headers: {},
        method: 'GET',
    }).then((r) => r.json().then((j) => (data.value = j)))
}
</script>

<template>
    <div>
        <button @click="fetchTables()">Get Tables</button>
    </div>
    <div v-if="data">
        {{ data }}
    </div>
</template>
