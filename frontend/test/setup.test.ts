import { describe, it, expect } from 'vitest'

describe('Test Setup', () => {
  it('should perform basic math operations', () => {
    expect(2 + 2).toBe(4)
    expect(10 - 5).toBe(5)
    expect(3 * 4).toBe(12)
    expect(8 / 2).toBe(4)
  })

  it('should handle string operations', () => {
    expect('hello').toBe('hello')
    expect('world').toContain('orl')
    expect('test').toHaveLength(4)
  })

  it('should work with arrays', () => {
    const arr = [1, 2, 3, 4, 5]
    expect(arr).toHaveLength(5)
    expect(arr).toContain(3)
    expect(arr[0]).toBe(1)
  })
})
