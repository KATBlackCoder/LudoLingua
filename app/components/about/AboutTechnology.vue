<template>
  <section>
    <h3 class="text-lg font-medium mb-2">Technology</h3>
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-2 mt-2">
      <UBadge color="primary" variant="soft" class="justify-center">Tauri</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Rust</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Nuxt 4</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">TypeScript</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Nuxt UI</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Tailwind CSS</UBadge>
    </div>
    <div class="mt-4">
      <UAlert
        color="warning"
        variant="soft"
        icon="i-heroicons-exclamation-triangle"
        title="Save settings to apply changes"
      >
        <template #description>
          <div class="text-sm">
            After choosing a different model or languages in <strong>Settings</strong>, click <strong>Save</strong>. If you don’t save, the configuration won’t be used by translation or connection tests.
          </div>
        </template>
      </UAlert>
    </div>

    <UCard class="mt-4">
      <template #header>
        <div class="font-medium">LLM Parameters (Ollama)</div>
      </template>
      <div class="space-y-3 text-sm">
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
            <UBadge color="primary" variant="soft">Recommended: 256</UBadge>
            <span class="ml-2 text-muted">Use 512 for longer paragraphs; smaller for short UI strings.</span>
          </div>
        </div>
        <UAlert color="neutral" variant="soft" icon="i-heroicons-information-circle" class="mt-2">
          <template #title>How it’s applied</template>
          <template #description>
            These settings are sent to Ollama as <code>ModelOptions</code> (<code>temperature</code>, <code>num_predict</code>) during generation.
          </template>
        </UAlert>
      </div>
    </UCard>

    <UCard class="mt-4">
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
        <UAlert color="neutral" variant="soft" icon="i-heroicons-information-circle">
          <template #description>
            Actual capacity depends on quantization, drivers, and model family. For most projects, a 7–8B Q4/Q5 model on an 8–12 GB GPU is sufficient.
          </template>
        </UAlert>
      </div>
    </UCard>
  </section>
</template>


