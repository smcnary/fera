#ifndef FERA_STM32_H
#define FERA_STM32_H

// Fera Embedded Library - STM32 Support
// For bare-metal ARM Cortex-M development

#include <stdint.h>

// Common register definitions
#define __IO volatile
#define __I  volatile const
#define __O  volatile

// Example: STM32F4 peripheral addresses (simplified)
#define PERIPH_BASE     0x40000000UL
#define AHB1PERIPH_BASE (PERIPH_BASE + 0x00020000UL)
#define GPIOA_BASE      (AHB1PERIPH_BASE + 0x0000UL)
#define RCC_BASE        (AHB1PERIPH_BASE + 0x3800UL)

typedef struct {
    __IO uint32_t MODER;
    __IO uint32_t OTYPER;
    __IO uint32_t OSPEEDR;
    __IO uint32_t PUPDR;
    __I  uint32_t IDR;
    __IO uint32_t ODR;
    __IO uint32_t BSRR;
    __IO uint32_t LCKR;
    __IO uint32_t AFR[2];
} GPIO_TypeDef;

typedef struct {
    __IO uint32_t CR;
    __IO uint32_t PLLCFGR;
    __IO uint32_t CFGR;
    __IO uint32_t CIR;
    __IO uint32_t AHB1RSTR;
    __IO uint32_t AHB2RSTR;
    __IO uint32_t AHB3RSTR;
    uint32_t RESERVED0;
    __IO uint32_t APB1RSTR;
    __IO uint32_t APB2RSTR;
    uint32_t RESERVED1[2];
    __IO uint32_t AHB1ENR;
} RCC_TypeDef;

#define GPIOA ((GPIO_TypeDef *) GPIOA_BASE)
#define RCC   ((RCC_TypeDef *) RCC_BASE)

// Delay function (busy-wait, for simple embedded apps)
static inline void delay(volatile uint32_t count) {
    while (count--) {
        __asm__ volatile ("nop");
    }
}

#endif // FERA_STM32_H

