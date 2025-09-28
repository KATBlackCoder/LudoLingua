<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-info-50 dark:bg-info-900/20 rounded-lg">
              <UIcon name="i-lucide-cloud" class="text-info w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">RunPod Configuration</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Cloud GPU instances for AI translation</p>
            </div>
          </div>
          <UBadge color="info" variant="soft" size="sm">
            <UIcon name="i-lucide-cloud" class="w-3 h-3 mr-1" />
            Cloud Setup
          </UBadge>
        </div>
      </template>

      <div class="space-y-6">
        <!-- Cloud-based Ollama Alert -->
        <UAlert 
          color="info" 
          variant="soft" 
          icon="i-lucide-cloud"
          title="Cloud-based Ollama"
          description="RunPod provides cloud GPU instances that can run Ollama models. This is ideal if you don't have a powerful local GPU or want to scale your translation workload."
        />

        <!-- Setup Instructions -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-settings" class="text-gray-500 w-4 h-4" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Setup Instructions</span>
            </div>
          </template>
          
          <div class="space-y-6">
            <!-- Step 1: Create RunPod Instance -->
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-info-100 dark:bg-info-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-info-600 dark:text-info-400">1</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-2">Create a RunPod Instance</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                  Choose a GPU instance with sufficient VRAM and storage for the models:
                </p>
                
                <div class="space-y-3">
                  <UAlert 
                    color="warning" 
                    variant="soft" 
                    icon="i-lucide-hard-drive"
                    title="Storage Requirements"
                    description="Minimum 60GB storage required for both models (qwen3:30b ~19GB + deepseek-r1:32b ~20GB + system overhead). Storage optimization: If you only need one model, you can remove one line from the container command and use 40GB storage instead (single model ~20GB + system overhead)."
                  />
                  
                  <UAlert 
                    color="error" 
                    variant="soft" 
                    icon="i-lucide-info"
                    title="These are suggestions, not requirements"
                    description="The GPU choices below are recommendations based on model sizes. You can choose any instance that meets your needs and budget. Smaller instances work fine with smaller models."
                  />
                  
                  <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
                    <UCard class="p-3 bg-success-50 dark:bg-success-900/20 border border-success-200 dark:border-success-800">
                      <div class="flex items-center gap-2 mb-2">
                        <UIcon name="i-lucide-zap" class="text-success w-4 h-4" />
                        <span class="font-medium text-success-800 dark:text-success-200">RTX 4090 (24GB)</span>
                      </div>
                      <p class="text-xs text-success-700 dark:text-success-300">Required for 30B+ models</p>
                    </UCard>
                    
                    <UCard class="p-3 bg-primary-50 dark:bg-primary-900/20 border border-primary-200 dark:border-primary-800">
                      <div class="flex items-center gap-2 mb-2">
                        <UIcon name="i-lucide-cpu" class="text-primary w-4 h-4" />
                        <span class="font-medium text-primary-800 dark:text-primary-200">RTX 3080/4080 (12GB)</span>
                      </div>
                      <p class="text-xs text-primary-700 dark:text-primary-300">For 14B models only</p>
                    </UCard>
                    
                    <UCard class="p-3 bg-neutral-50 dark:bg-neutral-900/20 border border-neutral-200 dark:border-neutral-800">
                      <div class="flex items-center gap-2 mb-2">
                        <UIcon name="i-lucide-cpu" class="text-neutral-600 dark:text-neutral-400 w-4 h-4" />
                        <span class="font-medium text-neutral-800 dark:text-neutral-200">RTX 3060 (12GB)</span>
                      </div>
                      <p class="text-xs text-neutral-700 dark:text-neutral-300">Not recommended for 30B+ models</p>
                    </UCard>
                  </div>
                </div>
              </div>
            </div>

            <USeparator />

            <!-- Step 2: Container Start Command -->
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-info-100 dark:bg-info-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-info-600 dark:text-info-400">2</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-2">Container Start Command</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                  Use this command to automatically install and configure Ollama with the required models:
                </p>
                
                <UCard class="bg-gray-50 dark:bg-gray-800 p-4">
                  <div class="flex items-start justify-between gap-2">
                    <pre class="text-xs overflow-x-auto flex-1 font-mono"><code>{{ containerCommand }}</code></pre>
                    <UTooltip text="Copy to clipboard" :content="{ side: 'left' }">
                      <UButton
                        :color="copied ? 'success' : 'neutral'"
                        variant="link"
                        size="sm"
                        :icon="copied ? 'i-lucide-copy-check' : 'i-lucide-copy'"
                        aria-label="Copy to clipboard"
                        @click="copy(containerCommand)"
                      />
                    </UTooltip>
                  </div>
                </UCard>
                
                <UAlert 
                  color="info" 
                  variant="soft" 
                  icon="i-lucide-info" 
                  class="mt-3"
                  title="Pre-configured Models"
                  description="This command installs the models that LudoLingua supports: qwen3:30b (256K context, superior Japanese→English) and deepseek-r1:32b (advanced reasoning, adult content). You'll select from these models in the LudoLingua settings. Single model option: To save storage space, you can remove either ollama pull qwen3:30b && or ollama pull deepseek-r1:32b && from the command above. This reduces storage requirement to 40GB."
                />
              </div>
            </div>

            <USeparator />

            <!-- Step 3: Single Model Setup -->
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-info-100 dark:bg-info-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-info-600 dark:text-info-400">3</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-2">Single Model Setup (Optional)</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                  If you only need one model to save storage space, use one of these modified commands:
                </p>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                  <UCard class="bg-gray-50 dark:bg-gray-800 p-3">
                    <div class="flex items-center gap-2 mb-2">
                      <UIcon name="i-lucide-brain" class="text-primary w-4 h-4" />
                      <span class="text-xs font-medium text-gray-700 dark:text-gray-300">Qwen3:30b only (40GB storage)</span>
                    </div>
                    <div class="flex items-start justify-between gap-2">
                      <pre class="text-xs overflow-x-auto flex-1 font-mono"><code>{{ singleModelCommandQwen3 }}</code></pre>
                      <UTooltip text="Copy to clipboard" :content="{ side: 'left' }">
                        <UButton
                          :color="copiedQwen3 ? 'success' : 'neutral'"
                          variant="link"
                          size="sm"
                          :icon="copiedQwen3 ? 'i-lucide-copy-check' : 'i-lucide-copy'"
                          aria-label="Copy to clipboard"
                          @click="copyQwen3(singleModelCommandQwen3)"
                        />
                      </UTooltip>
                    </div>
                  </UCard>
                  
                  <UCard class="bg-gray-50 dark:bg-gray-800 p-3">
                    <div class="flex items-center gap-2 mb-2">
                      <UIcon name="i-lucide-brain" class="text-info w-4 h-4" />
                      <span class="text-xs font-medium text-gray-700 dark:text-gray-300">DeepSeek-R1:32b only (40GB storage)</span>
                    </div>
                    <div class="flex items-start justify-between gap-2">
                      <pre class="text-xs overflow-x-auto flex-1 font-mono"><code>{{ singleModelCommandDeepSeek }}</code></pre>
                      <UTooltip text="Copy to clipboard" :content="{ side: 'left' }">
                        <UButton
                          :color="copiedDeepSeek ? 'success' : 'neutral'"
                          variant="link"
                          size="sm"
                          :icon="copiedDeepSeek ? 'i-lucide-copy-check' : 'i-lucide-copy'"
                          aria-label="Copy to clipboard"
                          @click="copyDeepSeek(singleModelCommandDeepSeek)"
                        />
                      </UTooltip>
                    </div>
                  </UCard>
                </div>
              </div>
            </div>

            <USeparator />

            <!-- Step 4: Get Pod ID -->
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-info-100 dark:bg-info-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-info-600 dark:text-info-400">4</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-2">Get Your Pod ID</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
                  Find your pod ID in the RunPod dashboard. It looks like:
                </p>
                
                <UCard class="bg-gray-50 dark:bg-gray-800 p-3">
                  <code class="text-sm font-mono">abc123def456</code>
                </UCard>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
                  This is the unique identifier for your pod (usually 12+ characters).
                </p>
              </div>
            </div>

            <USeparator />

            <!-- Step 5: Configure in LudoLingua -->
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-info-100 dark:bg-info-900/30 rounded-full flex items-center justify-center">
                <span class="text-sm font-semibold text-info-600 dark:text-info-400">5</span>
              </div>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white mb-2">Configure in LudoLingua</h4>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-3">In Settings, set:</p>
                
                <ul class="list-disc pl-5 space-y-1 text-sm mb-3">
                  <li><strong>Provider:</strong> RunPod</li>
                  <li><strong>Pod ID:</strong> <code>abc123def456</code> (just the pod ID, not the full URL)</li>
                  <li><strong>Model:</strong> Select from available models (Qwen3 30B or DeepSeek-R1 32B)</li>
                </ul>
                
                <UAlert 
                  color="info" 
                  variant="soft" 
                  icon="i-lucide-info"
                  title="Automatic URL Formatting"
                  description="LudoLingua automatically converts your pod ID to the full RunPod URL: https://abc123def456-11434.proxy.runpod.net"
                />
              </div>
            </div>
          </div>
        </UCard>

        <!-- Recommended RunPod Templates -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-server" class="text-gray-500 w-4 h-4" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Recommended RunPod Templates</span>
            </div>
          </template>
          
          <div class="space-y-4">
            <UAlert 
              color="info" 
              variant="soft" 
              icon="i-lucide-info"
              title="Template selection is not mandatory"
              description="These are just recommendations based on common use cases. You can choose any RunPod template that meets your needs and budget. Smaller instances work fine with smaller models."
            />
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <UCard class="p-4 bg-success-50 dark:bg-success-900/20 border border-success-200 dark:border-success-800">
              <div class="flex items-center gap-2 mb-3">
                <UBadge color="success" variant="soft" size="sm">High Performance</UBadge>
                <span class="font-medium text-gray-900 dark:text-white">RTX 4090</span>
              </div>
              <div class="space-y-2 text-sm">
                <div><strong>VRAM:</strong> 24GB</div>
                <div><strong>Storage:</strong> 60GB+ (recommended)</div>
                <div><strong>Models:</strong> Qwen3:30b, DeepSeek-R1:32b</div>
                <div><strong>Cost:</strong> ~$0.79/hour</div>
                <div><strong>Best for:</strong> Adult RPG translation, high-quality reasoning</div>
              </div>
            </UCard>
            
            <UCard class="p-4 bg-neutral-50 dark:bg-neutral-900/20 border border-neutral-200 dark:border-neutral-800">
              <div class="flex items-center gap-2 mb-3">
                <UBadge color="neutral" variant="soft" size="sm">Not Recommended</UBadge>
                <span class="font-medium text-gray-900 dark:text-white">RTX 3080/4080</span>
              </div>
              <div class="space-y-2 text-sm">
                <div><strong>VRAM:</strong> 12GB</div>
                <div><strong>Storage:</strong> 60GB+ (minimum)</div>
                <div><strong>Models:</strong> Insufficient VRAM for 30B+ models</div>
                <div><strong>Cost:</strong> ~$0.34/hour</div>
                <div><strong>Best for:</strong> Not suitable for 30B+ models</div>
              </div>
            </UCard>
            </div>
          </div>
        </UCard>

        <!-- Cost Optimization Tips -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-dollar-sign" class="text-gray-500 w-4 h-4" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Cost Optimization Tips</span>
            </div>
          </template>
          
          <div class="space-y-4">
            <ul class="list-disc pl-5 space-y-2 text-sm">
              <li><strong>Use spot instances:</strong> Save up to 80% on costs with RunPod spot pricing</li>
              <li><strong>Auto-shutdown:</strong> Configure auto-shutdown after inactivity to avoid unnecessary charges</li>
              <li><strong>Right-size your instance:</strong> Choose the minimum VRAM needed for your model</li>
              <li><strong>Storage optimization:</strong> Use exactly 60GB for both models or 40GB for single model to minimize costs</li>
              <li><strong>Batch processing:</strong> Process multiple translations in one session to maximize efficiency</li>
            </ul>
            
            <UAlert 
              color="info" 
              variant="soft" 
              icon="i-lucide-info"
              title="Persistent Storage"
              description="Consider using RunPod's persistent storage to keep your models and avoid re-downloading them each time you start a new pod."
            />
          </div>
        </UCard>

        <!-- Troubleshooting -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-wrench" class="text-gray-500 w-4 h-4" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Troubleshooting</span>
            </div>
          </template>
          
          <div class="space-y-4">
            <div>
              <h4 class="font-semibold text-gray-900 dark:text-white mb-2">Connection Issues</h4>
              <ul class="list-disc pl-5 space-y-1 text-sm">
                <li>Ensure your pod is running and Ollama is started</li>
                <li>Check that the pod ID is correct (just the ID, not the full URL)</li>
                <li>Wait 2-3 minutes after pod startup for Ollama to fully initialize</li>
                <li>Verify the pod ID matches exactly what's shown in RunPod dashboard</li>
              </ul>
            </div>
            
            <USeparator />
            
            <div>
              <h4 class="font-semibold text-gray-900 dark:text-white mb-2">Model Issues</h4>
              <ul class="list-disc pl-5 space-y-1 text-sm">
                <li>Check if both models (qwen3:30b and deepseek-r1:32b) were successfully pulled</li>
                <li>Ensure the model name matches exactly in LudoLingua settings</li>
                <li>Both models should be available in the LudoLingua model dropdown</li>
                <li>Verify you have sufficient VRAM (24GB+) for 30B+ models</li>
              </ul>
            </div>
          </div>
        </UCard>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
const { copy, copied } = useClipboard()
const { copy: copyQwen3, copied: copiedQwen3 } = useClipboard()
const { copy: copyDeepSeek, copied: copiedDeepSeek } = useClipboard()

const containerCommand = ref(`bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen3:30b &&
ollama pull deepseek-r1:32b &&
sleep infinity
"`)

const singleModelCommandQwen3 = ref(`bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull qwen3:30b &&
sleep infinity
"`)

const singleModelCommandDeepSeek = ref(`bash -c "
apt update && apt install -y curl lshw &&
curl -fsSL https://ollama.com/install.sh | sh &&
nohup ollama serve > /root/ollama.log 2>&1 &
sleep 60 &&
ollama pull deepseek-r1:32b &&
sleep infinity
"`)
</script>