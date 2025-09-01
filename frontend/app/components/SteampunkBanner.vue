<template>
  <div class="steampunk-banner">
    <!-- Banner Frame -->
    <div class="banner-frame">
      <!-- Dark Wood Base -->
      <div class="wood-base">
        <!-- Wood grain texture -->
        <div class="wood-grain"></div>
        
        <!-- Mounting Hardware -->
        <div class="mounting-screw top-left"></div>
        <div class="mounting-screw top-right"></div>
        <div class="mounting-screw bottom-left"></div>
        <div class="mounting-screw bottom-right"></div>
        
        <!-- Blue Enamel Plaque -->
        <div class="enamel-plaque">
          <!-- Plaque Content -->
          <div class="plaque-content">
            <div class="banner-text">
              {{ bannerText }}
            </div>
          </div>
          
          <!-- Decorative Elements -->
          <div class="plaque-decorations">
            <!-- Corner flourishes -->
            <div class="flourish top-left">
              <svg viewBox="0 0 20 20" class="flourish-svg">
                <path d="M2 2 C8 2, 12 6, 18 2 C12 8, 8 12, 2 18 C8 12, 12 8, 18 18 C12 12, 8 8, 2 2" 
                      fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
              </svg>
            </div>
            <div class="flourish top-right">
              <svg viewBox="0 0 20 20" class="flourish-svg">
                <path d="M18 2 C12 2, 8 6, 2 2 C8 8, 12 12, 18 18 C12 12, 8 8, 2 18 C8 12, 12 8, 18 2" 
                      fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
              </svg>
            </div>
            <div class="flourish bottom-left">
              <svg viewBox="0 0 20 20" class="flourish-svg">
                <path d="M2 18 C8 18, 12 14, 18 18 C12 12, 8 8, 2 2 C8 8, 12 12, 18 2 C12 8, 8 12, 2 18" 
                      fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
              </svg>
            </div>
            <div class="flourish bottom-right">
              <svg viewBox="0 0 20 20" class="flourish-svg">
                <path d="M18 18 C12 18, 8 14, 2 18 C8 12, 12 8, 18 2 C12 8, 8 12, 2 2 C8 8, 12 12, 18 18" 
                      fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
              </svg>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  bannerText: {
    type: String,
    default: 'Vigilance Maintained - Until the Next Challenge Arises'
  }
})
</script>

<style scoped>
.steampunk-banner {
  display: flex;
  justify-content: center;
  margin-bottom: 40px;
}

.banner-frame {
  position: relative;
  transform: perspective(800px) rotateX(2deg);
  transition: transform 0.3s ease;
}

.banner-frame:hover {
  transform: perspective(800px) rotateX(0deg) scale(1.02);
}

.wood-base {
  position: relative;
  background: 
    linear-gradient(145deg, 
      #3c2414 0%, 
      #4a2c18 25%,
      #5d3420 50%,
      #4a2c18 75%,
      #3c2414 100%);
  
  border: 3px solid #8B4513;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 
    0 6px 12px rgba(0, 0, 0, 0.4),
    inset 0 2px 4px rgba(255, 255, 255, 0.1),
    inset 0 -2px 4px rgba(0, 0, 0, 0.3);
}

.wood-grain {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    repeating-linear-gradient(90deg, 
      transparent 0px, 
      rgba(0, 0, 0, 0.1) 1px,
      transparent 2px,
      transparent 8px,
      rgba(0, 0, 0, 0.05) 9px,
      transparent 10px),
    repeating-linear-gradient(0deg, 
      transparent 0px, 
      rgba(255, 255, 255, 0.02) 1px,
      transparent 2px);
  border-radius: 9px;
  pointer-events: none;
}

.mounting-screw {
  position: absolute;
  width: 16px;
  height: 16px;
  background: 
    radial-gradient(circle at 30% 30%, #E5E5E5, #C0C0C0 40%, #A0A0A0 70%, #808080);
  border: 1px solid #606060;
  border-radius: 50%;
  box-shadow: 
    inset 0 1px 2px rgba(255, 255, 255, 0.4),
    0 2px 4px rgba(0, 0, 0, 0.3);
}

.mounting-screw::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 8px;
  height: 1px;
  background: #606060;
  transform: translate(-50%, -50%);
  box-shadow: 0 0 0 1px #606060;
}

.mounting-screw::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 1px;
  height: 8px;
  background: #606060;
  transform: translate(-50%, -50%);
}

.top-left { top: 6px; left: 6px; }
.top-right { top: 6px; right: 6px; }
.bottom-left { bottom: 6px; left: 6px; }
.bottom-right { bottom: 6px; right: 6px; }

.enamel-plaque {
  position: relative;
  background: 
    linear-gradient(145deg, 
      #1e40af 0%, 
      #2563eb 25%,
      #3b82f6 50%,
      #2563eb 75%,
      #1e40af 100%);
  
  border: 4px solid #C0C0C0;
  border-radius: 8px;
  padding: 16px 24px;
  
  /* Enamel-like finish */
  box-shadow: 
    inset 0 2px 4px rgba(255, 255, 255, 0.3),
    inset 0 -2px 4px rgba(0, 0, 0, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.2);
  
  /* Subtle enamel texture */
  background-image: 
    radial-gradient(circle at 25% 25%, rgba(255, 255, 255, 0.2) 0%, transparent 50%),
    radial-gradient(circle at 75% 75%, rgba(0, 0, 0, 0.1) 0%, transparent 50%);
}

.plaque-content {
  position: relative;
  z-index: 2;
}

.banner-text {
  font-family: 'Georgia', serif;
  font-size: 20px;
  font-weight: bold;
  color: #FFD700;
  text-align: center;
  text-shadow: 
    0 1px 0 rgba(0, 0, 0, 0.5),
    0 2px 4px rgba(0, 0, 0, 0.3),
    0 0 8px rgba(255, 215, 0, 0.3);
  
  letter-spacing: 1px;
  line-height: 1.3;
}

.plaque-decorations {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
}

.flourish {
  position: absolute;
  width: 20px;
  height: 20px;
  color: rgba(255, 215, 0, 0.4);
}

.flourish.top-left { top: 4px; left: 4px; }
.flourish.top-right { top: 4px; right: 4px; }
.flourish.bottom-left { bottom: 4px; left: 4px; }
.flourish.bottom-right { bottom: 4px; right: 4px; }

.flourish-svg {
  width: 100%;
  height: 100%;
}

/* Responsive design */
@media (max-width: 768px) {
  .wood-base {
    padding: 16px;
  }
  
  .enamel-plaque {
    padding: 12px 20px;
  }
  
  .banner-text {
    font-size: 18px;
    line-height: 1.4;
  }
  
  .mounting-screw {
    width: 14px;
    height: 14px;
  }
  
  .mounting-screw::before {
    width: 6px;
  }
  
  .mounting-screw::after {
    height: 6px;
  }
  
  .flourish {
    width: 16px;
    height: 16px;
  }
}

@media (max-width: 480px) {
  .wood-base {
    padding: 12px;
  }
  
  .enamel-plaque {
    padding: 10px 16px;
    border-width: 3px;
  }
  
  .banner-text {
    font-size: 16px;
    line-height: 1.5;
  }
  
  .mounting-screw {
    width: 12px;
    height: 12px;
  }
  
  .mounting-screw::before {
    width: 5px;
  }
  
  .mounting-screw::after {
    height: 5px;
  }
  
  .flourish {
    width: 14px;
    height: 14px;
  }
}
</style>