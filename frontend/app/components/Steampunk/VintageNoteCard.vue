<template>
  <div v-if="notes" class="vintage-note-card">
    <!-- Card Background -->
    <div class="card-body">

      

      
      <!-- Note Content -->
      <div class="note-content">
        <div class="note-header">
          <span class="note-title">Incident Log</span>
        </div>
        
        <div class="note-text">
          "{{ notes }}"
        </div>
        
        <div class="note-footer">
          <span class="timestamp">{{ formattedDate }}</span>
        </div>
      </div>
    </div>
    

  </div>
</template>

<script setup>
const props = defineProps({
  notes: {
    type: String,
    default: null
  },
  resetTimestamp: {
    type: String,
    default: null
  }
})

const formattedDate = computed(() => {
  if (!props.resetTimestamp) return ''
  
  return new Date(props.resetTimestamp).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long', 
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
})
</script>

<style scoped>
.vintage-note-card {
  position: relative;
  max-width: 1800px;
  margin: 0 auto 40px auto;
  transform: rotate(-1deg);
  transition: transform 0.3s ease;
}

.vintage-note-card:hover {
  transform: rotate(0deg) scale(1.02);
}

.card-body {
  position: relative;
  background-image: url('~/assets/images/worn-scroll-horizontal.png');
  background-size: contain;
  background-position: center;
  background-repeat: no-repeat;

  border: none;
  border-radius: 0;
  padding: 60px 120px;
  box-shadow: none;
  min-height: 300px;
  aspect-ratio: 1024/650;
}

.paper-texture {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    repeating-linear-gradient(0deg, 
      transparent, 
      transparent 1px, 
      rgba(139, 69, 19, 0.02) 1px, 
      rgba(139, 69, 19, 0.02) 2px),
    repeating-linear-gradient(90deg, 
      transparent, 
      transparent 1px, 
      rgba(139, 69, 19, 0.01) 1px, 
      rgba(139, 69, 19, 0.01) 2px);
  border-radius: 6px;
  pointer-events: none;
}

.corner-decoration {
  position: absolute;
  width: 16px;
  height: 16px;
  background: 
    radial-gradient(circle at center, #d4af37, #b8941f);
  border: 1px solid #8b7355;
  transform: rotate(45deg);
}

.corner-decoration::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 6px;
  height: 6px;
  background: #8b7355;
  border-radius: 50%;
  transform: translate(-50%, -50%);
}

.top-left { top: -8px; left: -8px; }
.top-right { top: -8px; right: -8px; }
.bottom-left { bottom: -8px; left: -8px; }
.bottom-right { bottom: -8px; right: -8px; }

.wax-seal {
  position: absolute;
  top: -12px;
  right: 20px;
  width: 40px;
  height: 40px;
  background: 
    radial-gradient(circle at 30% 30%, #cc0000, #aa0000 60%, #880000);
  border: 2px solid #660000;
  border-radius: 50%;
  box-shadow: 
    0 2px 4px rgba(0, 0, 0, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  
  display: flex;
  align-items: center;
  justify-content: center;
}

.seal-inner {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.seal-icon {
  width: 18px;
  height: 18px;
  color: rgba(255, 255, 255, 0.9);
  filter: drop-shadow(0 1px 1px rgba(0, 0, 0, 0.5));
}

.note-content {
  position: relative;
  z-index: 2;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  min-height: 200px;
}

.note-header {
  text-align: center;
  margin-bottom: 20px;
}

.note-title {
  font-family: 'Brush Script MT', 'Lucida Handwriting', cursive;
  font-size: 42px;
  font-weight: normal;
  color: #2b1810;
  text-shadow: 0px 0px 1px rgba(0, 0, 0, 0.1);
  letter-spacing: 1px;
}

.underline {
  width: 120px;
  height: 2px;
  background: linear-gradient(to right, transparent, #d4af37, transparent);
  margin: 8px auto 0;
  border-radius: 1px;
}

.note-text {
  font-family: 'Brush Script MT', 'Lucida Handwriting', cursive;
  font-size: 28px;
  line-height: 1.3;
  color: #2b1810;
  font-style: italic;
  font-weight: normal;
  margin: 15px 0 15px 0;
  padding: 0 60px;
  max-width: 100%;

  /* Limit to ~6 lines at this font size */
  display: -webkit-box;
  -webkit-line-clamp: 6;
  line-clamp: 6;
  -webkit-box-orient: vertical;
  overflow: hidden;

  /* Subtle ink effect */
  text-shadow: 0px 0px 1px rgba(0, 0, 0, 0.1);
}

.note-footer {
  margin-top: 15px;
  text-align: center;
  padding-top: 0;
  border-top: none;
}

.timestamp {
  font-family: 'Brush Script MT', 'Lucida Handwriting', cursive;
  font-size: 22px;
  color: #2b1810;
  font-style: italic;
  font-weight: normal;
  text-shadow: 0px 0px 1px rgba(0, 0, 0, 0.1);
}

/* Responsive design */
@media (max-width: 768px) {
  .card-body {
    padding: 40px 60px;
  }

  .note-title {
    font-size: 36px;
  }

  .note-text {
    font-size: 24px;
    padding: 0 40px;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .wax-seal {
    width: 32px;
    height: 32px;
    right: 16px;
  }

  .seal-inner {
    width: 20px;
    height: 20px;
  }

  .seal-icon {
    width: 14px;
    height: 14px;
  }
}

@media (max-width: 480px) {
  .vintage-note-card {
    transform: rotate(-0.5deg);
  }

  .card-body {
    padding: 30px 40px;
  }

  .note-title {
    font-size: 32px;
  }

  .note-text {
    font-size: 20px;
    padding: 0 30px;
    margin: 16px 0;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .timestamp {
    font-size: 18px;
  }

  .corner-decoration {
    width: 12px;
    height: 12px;
  }

  .corner-decoration::before {
    width: 4px;
    height: 4px;
  }
}
</style>