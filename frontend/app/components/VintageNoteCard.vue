<template>
  <div v-if="notes" class="vintage-note-card">
    <!-- Card Background -->
    <div class="card-body">
      <!-- Decorative Corner Elements -->
      <div class="corner-decoration top-left"></div>
      <div class="corner-decoration top-right"></div>
      <div class="corner-decoration bottom-left"></div>
      <div class="corner-decoration bottom-right"></div>
      
      <!-- Wax Seal -->
      <div class="wax-seal">
        <div class="seal-inner">
          <svg viewBox="0 0 24 24" class="seal-icon">
            <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" 
                  fill="currentColor"/>
          </svg>
        </div>
      </div>
      
      <!-- Note Content -->
      <div class="note-content">
        <div class="note-header">
          <span class="note-title">Incident Log</span>
          <div class="underline"></div>
        </div>
        
        <div class="note-text">
          "{{ notes }}"
        </div>
        
        <div class="note-footer">
          <span class="timestamp">{{ formattedDate }}</span>
        </div>
      </div>
    </div>
    
    <!-- Paper texture overlay -->
    <div class="paper-texture"></div>
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
  max-width: 500px;
  margin: 0 auto;
  transform: rotate(-1deg);
  transition: transform 0.3s ease;
}

.vintage-note-card:hover {
  transform: rotate(0deg) scale(1.02);
}

.card-body {
  position: relative;
  background: 
    linear-gradient(135deg, 
      #f7f3e9 0%, 
      #f0ead6 25%,
      #e8dcc0 50%,
      #f0ead6 75%,
      #f7f3e9 100%);
  
  border: 2px solid #d4af37;
  border-radius: 8px;
  padding: 32px 28px 24px 28px;
  box-shadow: 
    0 4px 8px rgba(0, 0, 0, 0.2),
    inset 0 1px 0 rgba(255, 255, 255, 0.5),
    0 8px 32px rgba(0, 0, 0, 0.1);
  
  /* Subtle paper aging */
  background-image: 
    radial-gradient(circle at 20% 20%, rgba(139, 69, 19, 0.03) 0%, transparent 50%),
    radial-gradient(circle at 80% 80%, rgba(139, 69, 19, 0.02) 0%, transparent 50%);
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
}

.note-header {
  text-align: center;
  margin-bottom: 20px;
}

.note-title {
  font-family: 'Georgia', serif;
  font-size: 24px;
  font-weight: bold;
  color: #4a3c28;
  text-shadow: 0 1px 2px rgba(255, 255, 255, 0.5);
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
  font-family: 'Georgia', serif;
  font-size: 18px;
  line-height: 1.6;
  color: #2c2416;
  text-align: center;
  font-style: italic;
  margin: 20px 0;
  padding: 0 12px;
  
  /* Ink-like text appearance */
  text-shadow: 0 0 1px rgba(44, 36, 22, 0.3);
}

.note-footer {
  margin-top: 24px;
  text-align: right;
  padding-top: 12px;
  border-top: 1px solid rgba(212, 175, 55, 0.3);
}

.timestamp {
  font-family: 'Georgia', serif;
  font-size: 14px;
  color: #6b5b47;
  font-style: italic;
}

/* Responsive design */
@media (max-width: 768px) {
  .card-body {
    padding: 28px 24px 20px 24px;
  }
  
  .note-title {
    font-size: 20px;
  }
  
  .note-text {
    font-size: 16px;
    padding: 0 8px;
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
    padding: 24px 20px 16px 20px;
  }
  
  .note-title {
    font-size: 18px;
  }
  
  .note-text {
    font-size: 14px;
    margin: 16px 0;
  }
  
  .timestamp {
    font-size: 12px;
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