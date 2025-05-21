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
        <a href="javascript:void(0)" @click="addTodo" class="action-icon-link">
          <i class="fa fa-plus text-green"></i>
        </a>
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

        <!-- 手动显示标题文本 -->
        <template v-slot:default>
          <div class="todo-title"></div>
        </template>

        <template v-slot:append>
          <a href="javascript:void(0)" @click="deleteTodo(todo.id)" class="delete-link">
            <i class="fa fa-trash text-red"></i>
          </a>
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

/* 输入框样式 */
.todo-input :deep(.v-input__control .v-field) {
  border-radius: 8px;
  transition: box-shadow 0.3s ease;
}

.todo-input :deep(.v-input__control .v-field--active) {
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

/* 列表项样式 */
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

/* 复选框定制 */
.todo-item .custom-checkbox :deep(.v-selection-control .v-input__control .v-field) {
  border-radius: 6px;
  height: 32px;
  width: 32px;
}

.todo-item .custom-checkbox :deep(.v-selection-control__input) {
  transform: scale(0.9);
}

/* 自定义标题样式 */
.todo-title {
  font-size: 15px;
  flex-grow: 1;
  word-break: break-word;
  line-height: 1.5;
}

/* 删除链接样式 */
.delete-link {
  text-decoration: none;
  color: inherit;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  transition: background-color 0.2s ease, color 0.2s ease;
  margin-left: 8px;
}

.delete-link i {
  font-size: 14px;
  color: red !important;
  transition: color 0.2s ease;
}

.delete-link:hover {
  background-color: rgba(255, 0, 0, 0.1);
  color: darkred;
}

/* 添加按钮链接样式 */
.action-icon-link {
  text-decoration: none;
  color: inherit;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  transition: background-color 0.2s ease, color 0.2s ease;
}

.action-icon-link:hover {
  background-color: rgba(0, 150, 0, 0.1);
  color: darkgreen;
}

/* 深色模式适配 */
.v-theme--dark .todo-list .todo-item {
  background-color: #2c2c2c;
}

.v-theme--dark .todo-list .todo-item:hover {
  background-color: #3a3a3a;
}

.v-theme--dark .delete-link:hover {
  background-color: rgba(255, 87, 34, 0.1); /* Material Red A700 */
  color: #ef5350;
}

.v-theme--dark .action-icon-link:hover {
  background-color: rgba(56, 142, 60, 0.1);
  color: #66bb6a;
}
</style>