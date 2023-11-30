<template>
  <div class="container">
    <Hat class="background-hat" />
    <div class="content">
      <h1 class="title">Image Resize Tool</h1>
      <div :class="{
        dropzone: true,
        loading: loading
      }" @dragenter="(<HTMLDivElement>$event.target).classList.add('active')"
        @dragleave="(<HTMLDivElement>$event.target).classList.remove('active')">
        <ImageIcon class="dropzone-background" />
        <div class="dropzone-cta">Drop your images here</div>
        <Hat class="dropzone-loading-icon" />
      </div>
      <div class="options">
        <div class="success-option" :class="{active: directory}">
          <FolderIcon class="success-option-icon" @click="openFolder" />
          <div class="success-option-message">View in folder</div>
        </div>
      </div>
    </div>
    <Logo class="logo" />
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { shell } from '@tauri-apps/api';
import ImageIcon from './assets/Image.svg';
import FolderIcon from './assets/Folder.svg';
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue';
import Hat from './assets/Hat.svg';
import Logo from './assets/Logo.svg'
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true

let directory = ref("");
let loading = ref(false);

listen('tauri://file-drop', async event => {
  const files = (event.payload as string[]);
  loading.value = true;
  directory.value = '';

  // Invoke the command
  invoke('image_to_webp', { files: files }).then((message) => { loading.value = false; directory.value = message as string });
})

const openFolder = () => {
  shell.open(directory.value);
}
</script>

<style scoped lang="scss">
@import url('https://fonts.googleapis.com/css2?family=Be+Vietnam+Pro:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap');

h1 {
  user-select: none;
  -webkit-user-select: none;
  cursor: default;
}

.container {
  display: flex;
  justify-content: center;
  position: relative;
  height: 100vh;
  overflow: hidden;
  min-height: 600px;
  min-width: 800px;
  .background-hat {
    position: absolute;
    height: 125%;
    right: 0;
    top: 50%;
    transform: translate(30%, -50%);
    fill: var(--secondaryColour);
    z-index: -1;
    pointer-events: none;
    opacity: .025;
  }

  .content {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 750px;
    gap: 2rem;

    .title {
      width: 100%;
      font-weight: 200;
      margin: 0;
      font-size: 2rem;
    }

    .dropzone {
      position: relative;
      height: 175px;
      background-color: var(--primaryColour);
      border: 1px dashed var(--secondaryColour);
      transition: background-color .3s ease-in-out;
      flex-shrink: 0;

      &.active {
        background-color: var(--secondaryColour);

        .dropzone-background {
          fill: var(--primaryColour);
        }

        .dropzone-cta {
          color: var(--primaryColour);
        }
      }

      &.loading {
        .dropzone-background, .dropzone-cta {
          display: none;
        }
        
        .dropzone-loading-icon {
          display: block;
        }
      }

      .dropzone-background {
        position: absolute;
        transform: translate(-50%, -50%);
        left: 50%;
        top: 50%;
        height: 80%;
        fill: var(--secondaryColour);
        opacity: .1;
        pointer-events: none;
        transition: fill .3s ease;
      }

      .dropzone-cta {
        position: absolute;
        transform: translate(-50%, -50%);
        left: 50%;
        top: 50%;
        font-size: 1.5rem;
        font-weight: 200;
        pointer-events: none;
        transition: color .3s ease;
      }

      .dropzone-loading-icon {
        display: none;
        pointer-events: none;
        position: absolute;
        fill: var(--secondaryColour);
        height: 75px;
        position: absolute;
        left: 50%;
        top: 50%;
        transform: translate(-50%, 50%);
        animation-name: hover;
        animation-duration: 3s;
        animation-iteration-count: infinite;
        animation-timing-function: ease-in-out;
      }
    }

    .options {
      display: flex;
      flex-direction: column;
      gap: .5rem;
      align-self: flex-end;

      .success-option {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        width: 125px;
        pointer-events: none;
        opacity: 0;
        transition: opacity .3s ease;

        .success-option-icon {
          fill: var(--secondaryColour);
          width: 50%;
          cursor: pointer;
        }

        .success-option-message {
          font-size: 1rem;
        }
        &.active {
          pointer-events: all;
          opacity: 100;
        }
      }
    }
  }

  .logo {
    fill: var(--secondaryColour);
    position: absolute;
    left: 2.5rem;
    bottom: 2.5rem;
    height: 3.5rem;
  }
}

@keyframes hover {
  0% {
    transform: translate(-50%, -50%);
  }

  25% {
    transform: translate(-50%, -45%);
  }

  50% {
    transform: translate(-50%, -50%);
  }

  75% {
    transform: translate(-50%, -55%);
  }

  100% {
    transform: translate(-50%, -50%);
  }
}
</style>