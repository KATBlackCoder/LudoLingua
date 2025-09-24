<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
              <UIcon name="i-lucide-cpu" class="text-primary w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Ollama Configuration</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Local AI model setup and optimization</p>
            </div>
          </div>
          <UBadge color="primary" variant="soft" size="sm">
            <UIcon name="i-lucide-home" class="w-3 h-3 mr-1" />
            Local Setup
          </UBadge>
        </div>
      </template>

      <div class="space-y-6">
        <!-- Simplified Configuration Alert -->
        <UAlert 
          color="info" 
          variant="soft" 
          icon="i-lucide-check-circle"
          title="Simplified Configuration"
          description="LudoLingua automatically uses http://localhost:11434 for Ollama. Just install Ollama locally (ollama.com) and select your model in Settings. No URL configuration needed!"
        />

        <!-- Quick Setup -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-zap" class="text-gray-500 w-4 h-4" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Quick Setup</span>
            </div>
          </template>
          
          <div class="space-y-4">
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-primary-100 dark:bg-primary-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-primary-600 dark:text-primary-400">1</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-1">Install Ollama</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  Download and install Ollama from 
                  <UButton 
                    variant="link" 
                    size="sm" 
                    color="primary"
                    to="https://ollama.com/" 
                    target="_blank"
                  >
                    ollama.com
                  </UButton>
                </p>
              </div>
            </div>

            <USeparator />

            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-primary-100 dark:bg-primary-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-primary-600 dark:text-primary-400">2</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-1">Pull a Model</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">Install a model for translation:</p>
                <UCard class="bg-gray-50 dark:bg-gray-800 p-3">
                  <code class="text-sm font-mono">ollama pull qwen2.5:14b-instruct-q5_K_M</code>
                </UCard>
              </div>
            </div>

            <USeparator />

            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-primary-100 dark:bg-primary-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-primary-600 dark:text-primary-400">3</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-1">Configure in LudoLingua</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">In Settings, select:</p>
                <ul class="list-disc pl-5 space-y-1 text-sm">
                  <li><strong>Provider:</strong> Ollama</li>
                  <li><strong>Model:</strong> Select your installed model</li>
                  <li><strong>Endpoint:</strong> Automatically set to localhost</li>
                </ul>
              </div>
            </div>
          </div>
        </UCard>


        <!-- Performance & Hardware -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-cpu" class="text-gray-500 w-4 h-4" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Performance & Hardware</span>
            </div>
          </template>
          
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <UCard class="p-4 bg-neutral-50 dark:bg-neutral-900/20 border border-neutral-200 dark:border-neutral-800">
                <div class="flex items-center gap-2 mb-3">
                  <UBadge color="neutral" variant="soft" size="sm">Baseline</UBadge>
                  <span class="text-sm font-medium text-gray-700 dark:text-gray-300">CPU‑only</span>
                </div>
                <ul class="list-disc pl-5 space-y-1 text-sm">
                  <li><strong>CPU:</strong> 6–8 physical cores</li>
                  <li><strong>RAM:</strong> 16–32 GB</li>
                  <li><strong>VRAM:</strong> N/A</li>
                  <li><strong>Models:</strong> 7B Q4 quant</li>
                  <li><strong>Parallelism:</strong> 1–2</li>
                  <li class="text-gray-500 dark:text-gray-400">Slower; good for small batches/UI text.</li>
                </ul>
              </UCard>
              
              <UCard class="p-4 bg-primary-50 dark:bg-primary-900/20 border border-primary-200 dark:border-primary-800">
                <div class="flex items-center gap-2 mb-3">
                  <UBadge color="primary" variant="soft" size="sm">Recommended</UBadge>
                  <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Single GPU</span>
                </div>
                <ul class="list-disc pl-5 space-y-1 text-sm">
                  <li><strong>CPU:</strong> 6+ physical cores</li>
                  <li><strong>RAM:</strong> 16–32 GB</li>
                  <li><strong>VRAM:</strong> 8–12 GB</li>
                  <li><strong>Models:</strong> 7–8B Q4–Q5 quant</li>
                  <li><strong>Parallelism:</strong> 2–3</li>
                  <li class="text-gray-500 dark:text-gray-400">Balanced throughput and quality for dialogue/editing.</li>
                </ul>
              </UCard>
              
              <UCard class="p-4 bg-success-50 dark:bg-success-900/20 border border-success-200 dark:border-success-800">
                <div class="flex items-center gap-2 mb-3">
                  <UBadge color="success" variant="soft" size="sm">High</UBadge>
                  <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Large VRAM</span>
                </div>
                <ul class="list-disc pl-5 space-y-1 text-sm">
                  <li><strong>CPU:</strong> 8+ physical cores</li>
                  <li><strong>RAM:</strong> 32+ GB</li>
                  <li><strong>VRAM:</strong> 16–24 GB</li>
                  <li><strong>Models:</strong> 13B Q4 or 7–8B higher quant</li>
                  <li><strong>Parallelism:</strong> 3–4</li>
                  <li class="text-gray-500 dark:text-gray-400">Best latency and longer generations (num_predict 512+).</li>
                </ul>
              </UCard>
            </div>
            
            <UAlert 
              color="neutral" 
              variant="soft" 
              icon="i-lucide-info"
              description="Actual capacity depends on quantization, drivers, and model family. For most projects, a 7–8B Q4/Q5 model on an 8–12 GB GPU is sufficient."
            />
          </div>
        </UCard>

        <!-- Recommended Models -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-brain" class="text-gray-500 w-4 h-4" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Recommended Models</span>
            </div>
          </template>
          
          <div class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <UCard class="p-4 bg-primary-50 dark:bg-primary-900/20 border border-primary-200 dark:border-primary-800">
                <div class="flex items-center gap-2 mb-3">
                  <UBadge color="primary" variant="soft" size="sm">Translation</UBadge>
                  <span class="font-medium text-gray-900 dark:text-white">Qwen2.5:14b</span>
                </div>
                <div class="space-y-2 text-sm">
                  <div><strong>Size:</strong> ~8GB (Q4 quantization)</div>
                  <div><strong>Best for:</strong> High-quality translations, dialogue, and complex text</div>
                  <div><strong>Requirements:</strong> 8GB+ VRAM or 16GB+ RAM</div>
                </div>
              </UCard>
              
              <UCard class="p-4 bg-neutral-50 dark:bg-neutral-900/20 border border-neutral-200 dark:border-neutral-800">
                <div class="flex items-center gap-2 mb-3">
                  <UBadge color="neutral" variant="soft" size="sm">Lightweight</UBadge>
                  <span class="font-medium text-gray-900 dark:text-white">Qwen2.5:7b</span>
                </div>
                <div class="space-y-2 text-sm">
                  <div><strong>Size:</strong> ~4GB (Q4 quantization)</div>
                  <div><strong>Best for:</strong> Quick translations, UI text, and basic content</div>
                  <div><strong>Requirements:</strong> 4GB+ VRAM or 8GB+ RAM</div>
                </div>
              </UCard>
            </div>
            
            <UAlert 
              color="info" 
              variant="soft" 
              icon="i-lucide-info"
              title="Model Installation"
              description="Install models using: ollama pull qwen2.5:14b or ollama pull qwen2.5:7b"
            />
          </div>
        </UCard>
      </div>
    </UCard>
  </div>
</template>
