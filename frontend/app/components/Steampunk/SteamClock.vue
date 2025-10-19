<template>
  <div class="steam-clock">
    <!-- Main Clock Frame -->
    <div class="clock-frame">
      <!-- Clock Face -->
      <div class="clock-face">
        <!-- Time Groups Container -->
        <div class="time-groups-container">
          <TimeGroup
            label="Years"
            :value="timeBreakdown.years"
            :has-higher-units="hasHigherUnits.years"
            key="years"
          />
          
          <TimeGroup
            label="Months"
            :value="timeBreakdown.months"
            :has-higher-units="hasHigherUnits.months"
            key="months"
          />
          
          <TimeGroup
            label="Weeks"
            :value="timeBreakdown.weeks"
            :has-higher-units="hasHigherUnits.weeks"
            key="weeks"
          />
          
          <TimeGroup
            label="Days"
            :value="timeBreakdown.days"
            :has-higher-units="hasHigherUnits.days"
            key="days"
          />
          
          <TimeGroup
            label="Hours"
            :value="timeBreakdown.hours"
            :has-higher-units="hasHigherUnits.hours"
            key="hours"
          />
          
          <TimeGroup
            label="Minutes"
            :value="timeBreakdown.minutes"
            :has-higher-units="hasHigherUnits.minutes"
            key="minutes"
          />
          
          <TimeGroup
            label="Seconds"
            :value="timeBreakdown.seconds"
            :has-higher-units="hasHigherUnits.seconds"
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
          
          <!-- Clockwork Gears - Only Tick -->
          <div class="gear gear-left">
            <img src="~/assets/images/gear1.png" alt="" class="gear-image" :style="{ transform: `rotate(${tickAngle}deg)` }" />
          </div>
          <div class="gear gear-right">
            <img src="~/assets/images/gear1.png" alt="" class="gear-image" :style="{ transform: `rotate(${tickAngle + 45}deg)` }" />
          </div>

          <!-- Minute Gears - Only Spin on Minute Changes -->
          <div class="minute-gear minute-gear-top">
            <img src="~/assets/images/gear2.png" alt="" class="gear-image" :style="{ transform: `rotate(${spinAngle}deg)` }" />
          </div>
          <div class="minute-gear minute-gear-bottom">
            <img src="~/assets/images/gear2.png" alt="" class="gear-image" :style="{ transform: `rotate(${-spinAngle}deg)` }" />
          </div>
        </div>
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

// Computed dictionary of which units have higher units
const hasHigherUnits = computed(() => {
  const { years, months, weeks, days, hours } = props.timeBreakdown
  const hasHigher = {}
  hasHigher.years = false
  hasHigher.months = years > 0
  hasHigher.weeks = hasHigher.months || months > 0
  hasHigher.days = hasHigher.weeks || weeks > 0
  hasHigher.hours = hasHigher.days || days > 0
  hasHigher.minutes = hasHigher.hours || hours > 0
  hasHigher.seconds = true
  return hasHigher
})

// Left/right gears - tick with seconds from time breakdown
const tickAngle = computed(() => {
  const seconds = props.timeBreakdown.seconds
  // Oscillate based on seconds for a "ticking" feel
  return (seconds % 60) * 6 // 360 degrees / 60 seconds = 6 degrees per second
})

// Top/bottom gears - spin based on time breakdown changes
const spinAngle = computed(() => {
  const minutes = props.timeBreakdown.minutes
  const seconds = props.timeBreakdown.seconds
  // Spin continuously with seconds, but make a full rotation each minute
  return (minutes * 360) + (seconds * 6)
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
    linear-gradient(145deg, #E5E5E5 0%, #C0C0C0 25%, #A8A8A8 50%, #C0C0C0 75%, #E5E5E5 100%);
  border: 12px solid #C0C0C0;
  border-radius: 20px;
  box-shadow: 
    inset 0 4px 8px rgba(255, 255, 255, 0.4),
    inset 0 -4px 8px rgba(0, 0, 0, 0.2),
    0 8px 32px rgba(0, 0, 0, 0.5);
  
  /* Solid silver - no texture */
}

.clock-face {
  position: relative;
  background: 
    radial-gradient(circle at center, 
      #1e3a8a 0%, 
      #1e40af 30%, 
      #1e3a8a 70%, 
      #0f172a 100%);
  border: none;
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
  justify-content: space-evenly;
  align-items: center;
  gap: 8px;
  height: 100%;
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

/* Clockwork Gears */
.gear {
  position: absolute;
  width: 100px;
  height: 100px;
  top: 50%;
  transform: translateY(-50%);

  display: flex;
  align-items: center;
  justify-content: center;
}

.gear-left { left: -50px; }
.gear-right { right: -50px; }

.gear-image {
  position: absolute;
  width: 100px;
  height: 100px;
  transition: transform 1s ease-in-out;
  filter: drop-shadow(0 6px 12px rgba(0, 0, 0, 0.4));
}

/* Minute Gears */
.minute-gear {
  position: absolute;
  width: 100px;
  height: 100px;

  display: flex;
  align-items: center;
  justify-content: center;
}

.minute-gear-top { 
  top: 15px; 
  left: 40%;
  transform: translateX(-50%);
}

.minute-gear-bottom { 
  bottom: 15px; 
  left: 60%;
  transform: translateX(-50%);
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
    height: 100%;
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
    height: 100%;
    gap: 6px;
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
    height: 100%;
    gap: 4px;
  }
  

  

}
</style>