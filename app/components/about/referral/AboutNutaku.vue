<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-secondary-50 dark:bg-secondary-900/20 rounded-lg">
              <UIcon name="i-lucide-gamepad-2" class="text-secondary w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Nutaku</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Adult gaming platform</p>
            </div>
          </div>
          <UBadge color="secondary" variant="soft" size="sm">
            <UIcon name="i-lucide-shield-alert" class="w-3 h-3 mr-1" />
            18+ Gaming
          </UBadge>
        </div>
      </template>

      <div class="space-y-6">
        <!-- Adult Content Warning -->
        <UAlert 
          color="warning" 
          variant="soft" 
          icon="i-lucide-shield-alert"
          title="Adult Content Warning"
          description="18+ Only: Nutaku contains adult content and is restricted to users 18 years or older. By proceeding, you confirm you are of legal age."
        />
        
        <!-- Description -->
        <div class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
          Nutaku is the world's largest adult gaming platform with over 100 million players enjoying high-quality adult games for PC, Android, and iOS. Join the community and discover premium adult gaming content.
        </div>

        <!-- Features Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <UCard class="p-4 bg-secondary-50 dark:bg-secondary-900/20 border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-secondary-100 dark:bg-secondary-900/30 rounded-lg">
                <UIcon name="i-lucide-gift" class="text-secondary w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-secondary-800 dark:text-secondary-200 mb-1">100 Free Gold</h4>
                <p class="text-xs text-secondary-700 dark:text-secondary-300">
                  Get 100 Nutaku Gold free when you sign up and set up your account
                </p>
              </div>
            </div>
          </UCard>
          
          <UCard class="p-4 bg-secondary-50 dark:bg-secondary-900/20 border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-secondary-100 dark:bg-secondary-900/30 rounded-lg">
                <UIcon name="i-lucide-users" class="text-secondary w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-secondary-800 dark:text-secondary-200 mb-1">100M+ Players</h4>
                <p class="text-xs text-secondary-700 dark:text-secondary-300">
                  Join the world's largest adult gaming community with 1000+ games
                </p>
              </div>
            </div>
          </UCard>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <UCard class="p-4 bg-secondary-50 dark:bg-secondary-900/20 border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-secondary-100 dark:bg-secondary-900/30 rounded-lg">
                <UIcon name="i-lucide-smartphone" class="text-secondary w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-secondary-800 dark:text-secondary-200 mb-1">Multi-Platform</h4>
                <p class="text-xs text-secondary-700 dark:text-secondary-300">
                  Play on PC, Android, and iOS with cross-platform compatibility
                </p>
              </div>
            </div>
          </UCard>
          
          <UCard class="p-4 bg-secondary-50 dark:bg-secondary-900/20 border border-secondary-200 dark:border-secondary-800">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-secondary-100 dark:bg-secondary-900/30 rounded-lg">
                <UIcon name="i-lucide-star" class="text-secondary w-4 h-4" />
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-secondary-800 dark:text-secondary-200 mb-1">Premium Quality</h4>
                <p class="text-xs text-secondary-700 dark:text-secondary-300">
                  High-quality adult games with regular updates and new releases
                </p>
              </div>
            </div>
          </UCard>
        </div>

        <!-- Referral Benefits Alert -->
        <UAlert 
          color="info" 
          variant="soft" 
          icon="i-lucide-info"
          title="Referral Benefits"
          description="Both you and your friend will earn free Nutaku Gold when you join Nutaku and set up your account using our referral link!"
        />
        
        <!-- Action Buttons -->
        <div class="flex flex-col sm:flex-row gap-3">
          <UButton
            color="secondary"
            variant="solid"
            size="lg"
            icon="i-lucide-external-link"
            class="flex-1"
            :to="referralUrl"
            target="_blank"
          >
            Visit Nutaku (18+)
          </UButton>
          
          <UButton
            color="neutral"
            variant="outline"
            size="lg"
            icon="i-lucide-copy"
            @click="copyReferralLink"
          >
            Copy Link
          </UButton>
        </div>
        
        <!-- Referral ID -->
        <div class="text-center">
          <div class="inline-flex items-center gap-2 px-3 py-1 bg-gray-100 dark:bg-gray-800 rounded-full">
            <UIcon name="i-lucide-hash" class="text-gray-500 w-3 h-3" />
            <span class="text-xs text-gray-500 dark:text-gray-400 font-mono">Referral ID: 1563308</span>
          </div>
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
const { showToast } = useAppToast()
const referralUrl = 'https://www.nutaku.net/fr/signup/invite/id/1563308/'

const copyReferralLink = async () => {
  try {
    await navigator.clipboard.writeText(referralUrl)
    showToast('Copied', 'Nutaku referral link copied to clipboard', 'success', 3000, 'i-lucide-copy')
  } catch (error) {
    console.error('Failed to copy link:', error)
    showToast('Error', 'Failed to copy referral link', 'error', 3000, 'i-lucide-alert-triangle')
  }
}
</script>
