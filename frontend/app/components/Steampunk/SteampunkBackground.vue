<template>
  <div class="steampunk-background">
    <!-- Subtle overlay for depth -->
    <div class="background-layer"></div>
    
    <!-- Animated Gears -->
    <div class="gears-container">
      <!-- Large Background Gear -->
      <div class="gear gear-large gear-1">
        <svg viewBox="0 0 100 100" class="gear-svg">
          <defs>
            <radialGradient id="gearGradient1" cx="30%" cy="30%">
              <stop offset="0%" style="stop-color:#C0C0C0"/>
              <stop offset="40%" style="stop-color:#A0A0A0"/>
              <stop offset="100%" style="stop-color:#808080"/>
            </radialGradient>
          </defs>
          <path d="M50 10 L55 5 L60 10 L65 5 L70 10 L75 15 L80 10 L85 15 L90 20 L95 25 L90 30 L95 35 L90 40 L85 45 L90 50 L85 55 L80 60 L85 65 L80 70 L75 75 L70 80 L75 85 L70 90 L65 85 L60 90 L55 85 L50 90 L45 85 L40 90 L35 85 L30 90 L25 85 L20 80 L15 75 L20 70 L15 65 L20 60 L15 55 L20 50 L15 45 L20 40 L15 35 L20 30 L15 25 L20 20 L25 15 L30 10 L25 5 L30 10 L35 5 L40 10 L45 5 Z" 
                fill="url(#gearGradient1)" 
                stroke="#606060" 
                stroke-width="1"/>
          <circle cx="50" cy="50" r="15" fill="#404040" stroke="#606060" stroke-width="1"/>
        </svg>
      </div>
      
      <!-- Medium Gear -->
      <div class="gear gear-medium gear-2">
        <svg viewBox="0 0 80 80" class="gear-svg">
          <defs>
            <radialGradient id="gearGradient2" cx="30%" cy="30%">
              <stop offset="0%" style="stop-color:#94a3b8"/>
              <stop offset="40%" style="stop-color:#64748b"/>
              <stop offset="100%" style="stop-color:#475569"/>
            </radialGradient>
          </defs>
          <path d="M40 8 L44 4 L48 8 L52 4 L56 8 L60 12 L64 8 L68 12 L72 16 L76 20 L72 24 L76 28 L72 32 L68 36 L72 40 L68 44 L64 48 L68 52 L64 56 L60 60 L56 64 L60 68 L56 72 L52 68 L48 72 L44 68 L40 72 L36 68 L32 72 L28 68 L24 72 L20 68 L16 64 L12 60 L8 56 L12 52 L8 48 L12 44 L8 40 L12 36 L8 32 L12 28 L8 24 L12 20 L16 16 L20 12 L24 8 L20 4 L24 8 L28 4 L32 8 L36 4 Z"
                fill="url(#gearGradient2)"
                stroke="#334155"
                stroke-width="1"/>
          <circle cx="40" cy="40" r="12" fill="#1e293b" stroke="#334155" stroke-width="1"/>
        </svg>
      </div>
      
      <!-- Small Gear -->
      <div class="gear gear-small gear-3">
        <svg viewBox="0 0 60 60" class="gear-svg">
          <defs>
            <radialGradient id="gearGradient3" cx="30%" cy="30%">
              <stop offset="0%" style="stop-color:#C0C0C0"/>
              <stop offset="40%" style="stop-color:#A0A0A0"/>
              <stop offset="100%" style="stop-color:#808080"/>
            </radialGradient>
          </defs>
          <path d="M30 6 L33 3 L36 6 L39 3 L42 6 L45 9 L48 6 L51 9 L54 12 L57 15 L54 18 L57 21 L54 24 L51 27 L54 30 L51 33 L48 36 L51 39 L48 42 L45 45 L42 48 L45 51 L42 54 L39 51 L36 54 L33 51 L30 54 L27 51 L24 54 L21 51 L18 54 L15 51 L12 48 L9 45 L6 42 L9 39 L6 36 L9 33 L6 30 L9 27 L6 24 L9 21 L6 18 L9 15 L12 12 L15 9 L18 6 L15 3 L18 6 L21 3 L24 6 L27 3 Z" 
                fill="url(#gearGradient3)" 
                stroke="#606060" 
                stroke-width="1"/>
          <circle cx="30" cy="30" r="8" fill="#404040" stroke="#606060" stroke-width="1"/>
        </svg>
      </div>
      
      <!-- Additional Small Gears -->
      <div class="gear gear-small gear-4">
        <svg viewBox="0 0 50 50" class="gear-svg">
          <defs>
            <radialGradient id="gearGradient4" cx="30%" cy="30%">
              <stop offset="0%" style="stop-color:#cbd5e1"/>
              <stop offset="40%" style="stop-color:#94a3b8"/>
              <stop offset="100%" style="stop-color:#64748b"/>
            </radialGradient>
          </defs>
          <path d="M25 5 L27 3 L29 5 L31 3 L33 5 L35 7 L37 5 L39 7 L41 9 L43 11 L41 13 L43 15 L41 17 L39 19 L41 21 L39 23 L37 25 L39 27 L37 29 L35 31 L33 33 L35 35 L33 37 L31 35 L29 37 L27 35 L25 37 L23 35 L21 37 L19 35 L17 37 L15 35 L13 33 L11 31 L9 29 L11 27 L9 25 L11 23 L9 21 L11 19 L9 17 L11 15 L9 13 L11 11 L13 9 L15 7 L17 5 L15 3 L17 5 L19 3 L21 5 L23 3 Z"
                fill="url(#gearGradient4)"
                stroke="#475569"
                stroke-width="1"/>
          <circle cx="25" cy="25" r="6" fill="#334155" stroke="#475569" stroke-width="1"/>
        </svg>
      </div>
      
      <div class="gear gear-small gear-5">
        <svg viewBox="0 0 45 45" class="gear-svg">
          <defs>
            <radialGradient id="gearGradient5" cx="30%" cy="30%">
              <stop offset="0%" style="stop-color:#C0C0C0"/>
              <stop offset="40%" style="stop-color:#A0A0A0"/>
              <stop offset="100%" style="stop-color:#808080"/>
            </radialGradient>
          </defs>
          <path d="M22.5 4 L24 2 L25.5 4 L27 2 L28.5 4 L30 6 L31.5 4 L33 6 L34.5 8 L36 10 L34.5 12 L36 14 L34.5 16 L33 18 L34.5 20 L33 22 L31.5 24 L33 26 L31.5 28 L30 30 L28.5 32 L30 34 L28.5 36 L27 34 L25.5 36 L24 34 L22.5 36 L21 34 L19.5 36 L18 34 L16.5 36 L15 34 L13.5 32 L12 30 L10.5 28 L12 26 L10.5 24 L12 22 L10.5 20 L12 18 L10.5 16 L12 14 L10.5 12 L12 10 L13.5 8 L15 6 L16.5 4 L15 2 L16.5 4 L18 2 L19.5 4 L21 2 Z" 
                fill="url(#gearGradient5)" 
                stroke="#606060" 
                stroke-width="1"/>
          <circle cx="22.5" cy="22.5" r="5" fill="#404040" stroke="#606060" stroke-width="1"/>
        </svg>
      </div>
    </div>
    
    <!-- Steam Pipes -->
    <div class="pipes-container">
      <div class="pipe pipe-horizontal pipe-1"></div>
      <div class="pipe pipe-vertical pipe-2"></div>
      <div class="pipe pipe-diagonal pipe-3"></div>
    </div>
  </div>
</template>

<script setup>
// No props needed - this is a pure decorative background
</script>

<style scoped>
.steampunk-background {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: -1;
  overflow: hidden;
  pointer-events: none;
}

.background-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;

  /* Subtle overlay for depth and atmosphere - nautical blue tones */
  background:
    radial-gradient(circle at 30% 30%, rgba(0, 0, 0, 0.1) 0%, transparent 60%),
    radial-gradient(circle at 70% 70%, rgba(30, 58, 138, 0.08) 0%, transparent 50%);

  opacity: 0.3;
}

.gears-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.gear {
  position: absolute;
  opacity: 0.15;
  filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.3));
}

.gear-svg {
  width: 100%;
  height: 100%;
}

/* Gear positioning and sizing */
.gear-large {
  width: 300px;
  height: 300px;
  top: 10%;
  right: -100px;
}

.gear-medium {
  width: 200px;
  height: 200px;
  bottom: 15%;
  left: -50px;
}

.gear-small {
  width: 120px;
  height: 120px;
}

.gear-3 {
  top: 30%;
  left: 20%;
}

.gear-4 {
  width: 100px;
  height: 100px;
  bottom: 30%;
  right: 15%;
}

.gear-5 {
  width: 90px;
  height: 90px;
  top: 60%;
  right: 30%;
}

/* Gear animations */
.gear-1 {
  animation: rotate-clockwise 60s linear infinite;
}

.gear-2 {
  animation: rotate-counter-clockwise 45s linear infinite;
}

.gear-3 {
  animation: rotate-clockwise 30s linear infinite;
}

.gear-4 {
  animation: rotate-counter-clockwise 35s linear infinite;
}

.gear-5 {
  animation: rotate-clockwise 25s linear infinite;
}

@keyframes rotate-clockwise {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes rotate-counter-clockwise {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(-360deg);
  }
}

/* Steam Pipes */
.pipes-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  opacity: 0.1;
}

.pipe {
  position: absolute;
  background: 
    linear-gradient(to bottom, #C0C0C0, #A0A0A0 20%, #808080 80%, #606060);
  border: 1px solid rgba(96, 96, 96, 0.5);
  border-radius: 4px;
  box-shadow: 
    inset 0 1px 0 rgba(255, 255, 255, 0.2),
    0 2px 4px rgba(0, 0, 0, 0.3);
}

.pipe-horizontal {
  width: 200px;
  height: 12px;
  top: 20%;
  left: 10%;
  transform: rotate(-5deg);
}

.pipe-vertical {
  width: 12px;
  height: 150px;
  top: 50%;
  right: 20%;
  transform: rotate(10deg);
}

.pipe-diagonal {
  width: 180px;
  height: 10px;
  bottom: 25%;
  left: 30%;
  transform: rotate(25deg);
}

/* Responsive adjustments */
@media (max-width: 1024px) {
  .gear-large {
    width: 250px;
    height: 250px;
    right: -80px;
  }
  
  .gear-medium {
    width: 160px;
    height: 160px;
    left: -40px;
  }
  
  .gear-small {
    width: 100px;
    height: 100px;
  }
  
  .gear-4 {
    width: 80px;
    height: 80px;
  }
  
  .gear-5 {
    width: 70px;
    height: 70px;
  }
}

@media (max-width: 768px) {
  .gear-large {
    width: 200px;
    height: 200px;
    right: -60px;
  }
  
  .gear-medium {
    width: 130px;
    height: 130px;
    left: -30px;
  }
  
  .gear-small {
    width: 80px;
    height: 80px;
  }
  
  .gear-4,
  .gear-5 {
    width: 60px;
    height: 60px;
  }
  
  .pipe-horizontal {
    width: 150px;
    height: 10px;
  }
  
  .pipe-vertical {
    width: 10px;
    height: 120px;
  }
  
  .pipe-diagonal {
    width: 140px;
    height: 8px;
  }
}

@media (max-width: 480px) {
  .gear {
    opacity: 0.08;
  }
  
  .gear-large {
    width: 150px;
    height: 150px;
    right: -40px;
  }
  
  .gear-medium {
    width: 100px;
    height: 100px;
    left: -20px;
  }
  
  .gear-small {
    width: 60px;
    height: 60px;
  }
  
  .gear-4,
  .gear-5 {
    width: 45px;
    height: 45px;
  }
  
  .pipes-container {
    opacity: 0.05;
  }
}
</style>