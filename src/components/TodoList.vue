<template>
  <div class="todo-container">
    <!-- 输入框 -->
    <v-text-field
      v-model="newTodo"
      label="添加待办事项"
      variant="outlined"
      clearable
      @keyup.enter="addTodo"
      class="todo-input"
    >
      <!-- 添加按钮图标 -->
      <template v-slot:append-inner>
        <v-btn icon size="small" @click="addTodo" variant="text" class="action-icon-btn">
          <i class="fa fa-plus text-green"></i>
        </v-btn>
      </template>
    </v-text-field>

    <!-- 待办列表 -->
    <v-list v-if="todos.length" class="todo-list">
      <v-list-item
        v-for="todo in todos"
        :key="todo.id"
        :title="todo.title"
        :class="{ completed: todo.completed }"
        :style="{
          textDecoration: todo.completed ? 'line-through' : 'none',
          color: todo.completed ? 'gray' : 'inherit'
        }"
        rounded="lg"
        border
        class="todo-item mb-2"
      >
        <template v-slot:prepend>
          <v-checkbox-btn
            :model-value="todo.completed"
            @update:model-value="() => toggleTodo(todo.id)"
            color="primary"
            hide-details
            density="compact"
            class="custom-checkbox"
          ></v-checkbox-btn>
        </template>

        <template v-slot:default>
          <v-list-item-title></v-list-item-title>
        </template>

        <template v-slot:append>
          <v-btn icon size="small" @click="deleteTodo(todo.id)" variant="text" class="delete-icon-btn">
            <i class="fa fa-trash text-red"></i>
          </v-btn>
        </template>
      </v-list-item>
    </v-list>

    <!-- 空状态提示 -->
    <v-alert v-else type="info" text-align="center" class="mt-4">
      还没有待办事项，添加一个吧！
    </v-alert>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'


const todos = ref([])
const newTodo = ref('')

async function loadTodos() {
  try {
    todos.value = await invoke('get_todos')
  } catch (err) {
    console.error(err)
  }
}

loadTodos()

async function addTodo() {
  const title = newTodo.value.trim()
  if (!title) return

  try {
    await invoke('add_todo', { title })
    await loadTodos()
    newTodo.value = ''
  } catch (err) {
    console.error(err)
  }
}

async function toggleTodo(id) {
  try {
    await invoke('toggle_todo', { id })
    await loadTodos()
  } catch (err) {
    console.error(err)
  }
}

async function deleteTodo(id) {
  try {
    await invoke('delete_todo', { id })
    await loadTodos()
  } catch (err) {
    console.error(err)
  }
}
</script>

<style scoped>
.todo-container {
  max-width: 600px;
  margin: auto;
  padding: 1rem;
}

.todo-input :deep(.v-input__control .v-field) {
  border-radius: 8px;
  transition: box-shadow 0.3s ease;
}

.todo-input :deep(.v-input__control .v-field--active) {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.todo-list .todo-item {
  background-color: #f9f9f9;
  border-radius: 8px;
  transition: background-color 0.3s ease, box-shadow 0.3s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.todo-list .todo-item:hover {
  background-color: #f0f0f0;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.todo-item .custom-checkbox :deep(.v-selection-control .v-input__control .v-field) {
  border-radius: 6px;
  height: 32px;
  width: 32px;
}

.todo-item .custom-checkbox :deep(.v-selection-control__input) {
  transform: scale(0.9);
}

.delete-icon-btn i {
  color: red !important;
  transition: color 0.2s ease;
}

.delete-icon-btn:hover i {
  color: darkred !important;
}

/* 深色模式适配 */
.v-theme--dark .todo-list .todo-item {
  background-color: #2c2c2c;
}

.v-theme--dark .todo-list .todo-item:hover {
  background-color: #3a3a3a;
}
</style>