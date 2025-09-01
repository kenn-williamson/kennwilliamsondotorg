<template>
  <div class="steam-clock">
    <!-- Main Clock Frame -->
    <div class="clock-frame">
      <!-- Clock Face -->
      <div class="clock-face">
        <!-- Time Groups Container -->
        <div class="time-groups-container">
          <SlidingTimeGroup
            v-if="timeBreakdown.years > 0"
            label="Y"
            :value="timeBreakdown.years"
            key="years"
          />
          
          <SlidingTimeGroup
            v-if="timeBreakdown.months > 0 || timeBreakdown.years > 0"
            label="M"
            :value="timeBreakdown.months"
            key="months"
          />
          
          <SlidingTimeGroup
            v-if="timeBreakdown.weeks > 0 || timeBreakdown.months > 0 || timeBreakdown.years > 0"
            label="W"
            :value="timeBreakdown.weeks"
            key="weeks"
          />
          
          <SlidingTimeGroup
            v-if="timeBreakdown.days > 0 || timeBreakdown.weeks > 0 || timeBreakdown.months > 0 || timeBreakdown.years > 0"
            label="D"
            :value="timeBreakdown.days"
            key="days"
          />
          
          <SlidingTimeGroup
            label="H"
            :value="timeBreakdown.hours"
            key="hours"
          />
          
          <SlidingTimeGroup
            label="M"
            :value="timeBreakdown.minutes"
            key="minutes"
          />
          
          <SlidingTimeGroup
            label="S"
            :value="timeBreakdown.seconds"
            key="seconds"
          />
        </div>
        
        <!-- Decorative Elements -->
        <div class="clock-decorations">
          <!-- Corner Rivets -->
          <div class="rivet top-left"></div>
          <div class="rivet top-right"></div>
          <div class="rivet bottom-left"></div>
          <div class="rivet bottom-right"></div>
          
          <!-- Side Gauges -->
          <div class="gauge gauge-left">
            <div class="gauge-needle" :style="{ transform: `rotate(${gaugeAngle}deg)` }"></div>
          </div>
          <div class="gauge gauge-right">
            <div class="gauge-needle" :style="{ transform: `rotate(${gaugeAngle + 45}deg)` }"></div>
          </div>
        </div>
      </div>
      
      <!-- Steam Pipes -->
      <div class="steam-pipes">
        <div class="pipe pipe-left"></div>
        <div class="pipe pipe-right"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  timeBreakdown: {
    type: Object,
    required: true,
    default: () => ({
      years: 0,
      months: 0,
      weeks: 0,
      days: 0,
      hours: 0,
      minutes: 0,
      seconds: 0
    })
  }
})

// Animated gauge needle based on total seconds
const gaugeAngle = computed(() => {
  const totalSeconds = props.timeBreakdown.seconds + 
                      (props.timeBreakdown.minutes * 60) + 
                      (props.timeBreakdown.hours * 3600)
  
  // Oscillate the gauge based on seconds for a "living" feel
  return (totalSeconds % 60) * 6 // 360 degrees / 60 seconds = 6 degrees per second
})
</script>

<style scoped>
.steam-clock {
  display: flex;
  flex-direction: column;
  align-items: center;
  perspective: 1000px;
}

.clock-frame {
  position: relative;
  padding: 40px;
  background: 
    linear-gradient(145deg, #8B4513 0%, #A0522D 25%, #CD853F 50%, #A0522D 75%, #8B4513 100%);
  border: 6px solid #C0C0C0;
  border-radius: 20px;
  box-shadow: 
    inset 0 4px 8px rgba(255, 255, 255, 0.2),
    inset 0 -4px 8px rgba(0, 0, 0, 0.4),
    0 8px 32px rgba(0, 0, 0, 0.5),
    0 0 0 2px #FFD700;
  
  /* Wood grain texture */
  background-image: 
    repeating-linear-gradient(90deg, 
      transparent, transparent 2px, 
      rgba(0,0,0,0.1) 2px, rgba(0,0,0,0.1) 4px);
}

.clock-face {
  position: relative;
  background: 
    radial-gradient(circle at center, 
      #1e3a8a 0%, 
      #1e40af 30%, 
      #1e3a8a 70%, 
      #0f172a 100%);
  border: 4px solid #C0C0C0;
  border-radius: 12px;
  padding: 32px 24px;
  min-height: 200px;
  
  /* Subtle metallic sheen */
  box-shadow: 
    inset 0 2px 4px rgba(255, 255, 255, 0.1),
    inset 0 -2px 4px rgba(0, 0, 0, 0.3),
    0 4px 16px rgba(0, 0, 0, 0.3);
}

.clock-face::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 30% 30%, rgba(255, 255, 255, 0.1) 0%, transparent 60%),
    radial-gradient(circle at 70% 70%, rgba(0, 0, 0, 0.2) 0%, transparent 60%);
  border-radius: 8px;
  pointer-events: none;
}

.time-groups-container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  align-items: flex-end;
  gap: 8px;
  min-height: 140px;
  position: relative;
  z-index: 2;
}

.clock-decorations {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
}

.rivet {
  position: absolute;
  width: 16px;
  height: 16px;
  background: 
    radial-gradient(circle at 30% 30%, #E5E5E5, #C0C0C0 40%, #A0A0A0);
  border: 2px solid #808080;
  border-radius: 50%;
  box-shadow: 
    inset 0 1px 2px rgba(255, 255, 255, 0.5),
    0 2px 4px rgba(0, 0, 0, 0.3);
}

.rivet.top-left { top: 8px; left: 8px; }
.rivet.top-right { top: 8px; right: 8px; }
.rivet.bottom-left { bottom: 8px; left: 8px; }
.rivet.bottom-right { bottom: 8px; right: 8px; }

.gauge {
  position: absolute;
  width: 40px;
  height: 40px;
  border: 3px solid #C0C0C0;
  border-radius: 50%;
  background: 
    radial-gradient(circle at center, #2a2a2a 0%, #1a1a1a 100%);
  top: 50%;
  transform: translateY(-50%);
  
  display: flex;
  align-items: center;
  justify-content: center;
}

.gauge-left { left: -20px; }
.gauge-right { right: -20px; }

.gauge-needle {
  width: 2px;
  height: 14px;
  background: #FFD700;
  border-radius: 1px;
  transform-origin: center bottom;
  transition: transform 1s ease-in-out;
  box-shadow: 0 0 4px rgba(255, 215, 0, 0.5);
}

.steam-pipes {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 100%;
  height: 8px;
  pointer-events: none;
}

.pipe {
  position: absolute;
  width: 60px;
  height: 8px;
  background: linear-gradient(to bottom, #C0C0C0, #A0A0A0);
  border: 1px solid #808080;
  border-radius: 4px;
}

.pipe-left { 
  left: -50px;
  transform: rotate(-15deg);
}

.pipe-right { 
  right: -50px;
  transform: rotate(15deg);
}

/* Responsive design */
@media (max-width: 1024px) {
  .clock-frame {
    padding: 32px;
  }
  
  .clock-face {
    padding: 24px 20px;
    min-height: 160px;
  }
  
  .time-groups-container {
    min-height: 120px;
  }
}

@media (max-width: 768px) {
  .clock-frame {
    padding: 24px;
    border-width: 4px;
  }
  
  .clock-face {
    padding: 20px 16px;
    min-height: 140px;
  }
  
  .time-groups-container {
    min-height: 100px;
    gap: 6px;
  }
  
  .gauge {
    width: 32px;
    height: 32px;
  }
  
  .gauge-needle {
    height: 12px;
  }
  
  .rivet {
    width: 12px;
    height: 12px;
  }
}

@media (max-width: 480px) {
  .clock-frame {
    padding: 20px;
    border-width: 3px;
  }
  
  .clock-face {
    padding: 16px 12px;
    min-height: 120px;
  }
  
  .time-groups-container {
    min-height: 80px;
    gap: 4px;
  }
  
  .gauge {
    width: 28px;
    height: 28px;
  }
  
  .gauge-needle {
    height: 10px;
  }
  
  .pipe {
    width: 40px;
    height: 6px;
  }
}
</style>