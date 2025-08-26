<template>
  <div class="counter-banner">
    <div class="banner-container">
      <h2 class="banner-title">Time since last incident</h2>
      <div class="counter-display">
        <div v-for="(unit, index) in timeUnits" :key="index" class="time-unit">
          <span class="time-value">{{ unit.value }}</span>
          <span class="time-label">{{ unit.label }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface TimeUnit {
  value: number
  label: string
}

const timeUnits = ref<TimeUnit[]>([])
let intervalId: number | null = null

const startDate = new Date('2025-08-24T00:00:00')

const calculateTimeDifference = () => {
  const now = new Date()
  
  if (now < startDate) {
    timeUnits.value = [{ value: 0, label: 'seconds' }]
    return
  }
  
  // Calculate years
  let years = now.getFullYear() - startDate.getFullYear()
  
  // Calculate months
  let months = now.getMonth() - startDate.getMonth()
  if (months < 0) {
    years--
    months += 12
  }
  
  // Calculate days
  let days = now.getDate() - startDate.getDate()
  if (days < 0) {
    months--
    if (months < 0) {
      years--
      months += 12
    }
    // Get the last day of the previous month
    const lastMonth = new Date(now.getFullYear(), now.getMonth(), 0)
    days += lastMonth.getDate()
  }
  
  // Calculate weeks and remaining days
  const weeks = Math.floor(days / 7)
  days = days % 7
  
  // Calculate hours, minutes, seconds
  const hours = now.getHours() - startDate.getHours()
  const minutes = now.getMinutes() - startDate.getMinutes()
  const seconds = now.getSeconds() - startDate.getSeconds()
  
  // Adjust for negative values (simplified since we're starting at midnight)
  const totalHours = hours >= 0 ? hours : 24 + hours
  const totalMinutes = minutes >= 0 ? minutes : 60 + minutes
  const totalSeconds = seconds >= 0 ? seconds : 60 + seconds
  
  // Build display array with only non-zero values
  const units: TimeUnit[] = []
  
  if (years > 0) {
    units.push({ value: years, label: years === 1 ? 'year' : 'years' })
  }
  if (months > 0) {
    units.push({ value: months, label: months === 1 ? 'month' : 'months' })
  }
  if (weeks > 0) {
    units.push({ value: weeks, label: weeks === 1 ? 'week' : 'weeks' })
  }
  if (days > 0) {
    units.push({ value: days, label: days === 1 ? 'day' : 'days' })
  }
  if (totalHours > 0) {
    units.push({ value: totalHours, label: totalHours === 1 ? 'hour' : 'hours' })
  }
  if (totalMinutes > 0) {
    units.push({ value: totalMinutes, label: totalMinutes === 1 ? 'minute' : 'minutes' })
  }
  
  // Always show seconds
  units.push({ value: totalSeconds, label: totalSeconds === 1 ? 'second' : 'seconds' })
  
  timeUnits.value = units
}

onMounted(() => {
  calculateTimeDifference()
  intervalId = window.setInterval(calculateTimeDifference, 1000)
})

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
})
</script>

<style scoped>
.counter-banner {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 2rem 1rem;
}

.banner-container {
  max-width: 800px;
  width: 100%;
  background: var(--color-background-soft);
  border: 2px solid var(--color-border);
  border-radius: 12px;
  padding: 2rem;
  text-align: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.banner-title {
  font-size: 1.8rem;
  font-weight: bold;
  color: var(--color-heading);
  margin: 0 0 1.5rem 0;
}

.counter-display {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 1.5rem;
  align-items: center;
}

.time-unit {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 80px;
  padding: 0.5rem;
}

.time-value {
  font-size: 2rem;
  font-weight: bold;
  color: var(--color-text);
  line-height: 1;
}

.time-label {
  font-size: 0.9rem;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-top: 0.25rem;
}

@media (max-width: 768px) {
  .banner-container {
    padding: 1.5rem;
  }
  
  .banner-title {
    font-size: 1.5rem;
  }
  
  .counter-display {
    gap: 1rem;
  }
  
  .time-value {
    font-size: 1.5rem;
  }
  
  .time-unit {
    min-width: 60px;
  }
}

@media (max-width: 480px) {
  .banner-title {
    font-size: 1.2rem;
  }
  
  .counter-display {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .time-value {
    font-size: 1.2rem;
  }
  
  .time-label {
    font-size: 0.8rem;
  }
  
  .time-unit {
    min-width: 50px;
  }
}
</style>