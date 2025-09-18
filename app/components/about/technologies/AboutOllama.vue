<template>
  <section>
    <h3 class="text-lg font-medium mb-4">Ollama Configuration</h3>
    
    <UAlert
      color="error"
      variant="soft"
      icon="i-lucide-triangle-alert"
      class="mb-4"
      title="Model requirement"
    >
      <template #description>
        <div class="text-sm">
          You need an Ollama endpoint to translate. Either install Ollama locally and use
          <code>http://localhost:11434</code> (<a class="underline" href="https://ollama.com/" target="_blank" rel="noreferrer">ollama.com</a>),
          or point to a remote Ollama-compatible URL (e.g., a hosted instance).
          Configure it under <strong>Settings</strong>.
        </div>
      </template>
    </UAlert>

    <UCard class="mb-4">
      <template #header>
        <div class="font-medium">LLM Parameters</div>
      </template>
      <div class="space-y-3 text-sm">
        <UAlert color="warning" variant="soft" icon="i-lucide-settings">
          <template #title>Connection tests require saved settings</template>
          <template #description>
            Tests only run once your settings file exists. Open <strong>Settings</strong> and click <strong>Save</strong> at least once to create <code>ludollingua-settings.json</code>.
          </template>
        </UAlert>
        <div>
          <div class="font-medium">Temperature</div>
          <div class="text-muted">Controls randomness/creativity. Lower = more literal and consistent; higher = more diverse but riskier.</div>
          <div class="mt-1"><UBadge color="primary" variant="soft">Recommended: 0.2–0.3 (default: 0.3)</UBadge></div>
        </div>
        <USeparator />
        <div>
          <div class="font-medium">Max Tokens (num_predict)</div>
          <div class="text-muted">Caps the length of the generated translation. Prevents overly long outputs and improves latency.</div>
          <div class="mt-1">
            <UBadge color="primary" variant="soft">Recommended: 512</UBadge>
            <span class="ml-2 text-muted">Use 1024 for longer paragraphs; smaller for short UI strings.</span>
          </div>
        </div>
        <UAlert color="neutral" variant="soft" icon="i-lucide-info" class="mt-2">
          <template #title>How it's applied</template>
          <template #description>
            These settings are sent to Ollama as <code>ModelOptions</code> (<code>temperature</code>, <code>num_predict</code>) during generation.
          </template>
        </UAlert>
      </div>
    </UCard>

    <UCard class="mb-4">
      <template #header>
        <div class="font-medium">Performance & Hardware (Guidance)</div>
      </template>
      <div class="space-y-4 text-sm">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
          <UCard>
            <template #header>
              <div class="flex items-center gap-2"><UBadge color="neutral" variant="soft">Baseline</UBadge><span>CPU‑only</span></div>
            </template>
            <ul class="list-disc pl-5 space-y-1">
              <li><strong>CPU</strong>: 6–8 physical cores</li>
              <li><strong>RAM</strong>: 16–32 GB</li>
              <li><strong>VRAM</strong>: N/A</li>
              <li><strong>Models</strong>: 7B Q4 quant</li>
              <li><strong>Parallelism</strong>: 1–2</li>
              <li class="text-muted">Slower; good for small batches/UI text.</li>
            </ul>
          </UCard>
          <UCard>
            <template #header>
              <div class="flex items-center gap-2"><UBadge color="primary" variant="soft">Recommended</UBadge><span>Single GPU</span></div>
            </template>
            <ul class="list-disc pl-5 space-y-1">
              <li><strong>CPU</strong>: 6+ physical cores</li>
              <li><strong>RAM</strong>: 16–32 GB</li>
              <li><strong>VRAM</strong>: 8–12 GB</li>
              <li><strong>Models</strong>: 7–8B Q4–Q5 quant</li>
              <li><strong>Parallelism</strong>: 2–3</li>
              <li class="text-muted">Balanced throughput and quality for dialogue/editing.</li>
            </ul>
          </UCard>
          <UCard>
            <template #header>
              <div class="flex items-center gap-2"><UBadge color="success" variant="soft">High</UBadge><span>Large VRAM</span></div>
            </template>
            <ul class="list-disc pl-5 space-y-1">
              <li><strong>CPU</strong>: 8+ physical cores</li>
              <li><strong>RAM</strong>: 32+ GB</li>
              <li><strong>VRAM</strong>: 16–24 GB</li>
              <li><strong>Models</strong>: 13B Q4 or 7–8B higher quant</li>
              <li><strong>Parallelism</strong>: 3–4</li>
              <li class="text-muted">Best latency and longer generations (num_predict 512+).</li>
            </ul>
          </UCard>
        </div>
        <UAlert color="neutral" variant="soft" icon="i-lucide-info">
          <template #description>
            Actual capacity depends on quantization, drivers, and model family. For most projects, a 7–8B Q4/Q5 model on an 8–12 GB GPU is sufficient.
          </template>
        </UAlert>
      </div>
    </UCard>

    <UCard>
      <template #header>
        <div class="font-medium">Recommended Models</div>
      </template>
      <div class="space-y-3 text-sm">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UBadge color="primary" variant="soft">Translation</UBadge>
                <span class="font-medium">Qwen2.5:14b</span>
              </div>
            </template>
            <div class="space-y-2">
              <div><strong>Size:</strong> ~8GB (Q4 quantization)</div>
              <div><strong>Best for:</strong> High-quality translations, dialogue, and complex text</div>
              <div><strong>Requirements:</strong> 8GB+ VRAM or 16GB+ RAM</div>
            </div>
          </UCard>
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UBadge color="neutral" variant="soft">Lightweight</UBadge>
                <span class="font-medium">Qwen2.5:7b</span>
              </div>
            </template>
            <div class="space-y-2">
              <div><strong>Size:</strong> ~4GB (Q4 quantization)</div>
              <div><strong>Best for:</strong> Quick translations, UI text, and basic content</div>
              <div><strong>Requirements:</strong> 4GB+ VRAM or 8GB+ RAM</div>
            </div>
          </UCard>
        </div>
        <UAlert color="info" variant="soft" icon="i-lucide-info">
          <template #title>Model Installation</template>
          <template #description>
            Install models using: <code>ollama pull qwen2.5:14b</code> or <code>ollama pull qwen2.5:7b</code>
          </template>
        </UAlert>
      </div>
    </UCard>
  </section>
</template>
