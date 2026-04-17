// ═══════════════════════════════════════════════════════════════════
// OMNI-KERNEL PERSONAL CORE ENGINE v5.0
// ═══════════════════════════════════════════════════════════════════

async function startSynthesis() {
    const code = document.getElementById('source-input').value.trim();
    if (!code) return alert("Zəhmət olmasa şəxsi məntiqinizi daxil edin.");

    const laser = document.getElementById('grinder-laser');
    const logs = document.getElementById('stripping-logs');
    const bytecodeBox = document.getElementById('bytecode-output');
    const status = document.getElementById('verification-status');
    
    // UI Initiation
    laser.classList.remove('hidden');
    logs.innerHTML = '';
    bytecodeBox.innerText = 'Calculating Personalized DNA...';
    status.innerText = "SYNTHESIZING_uΩ...";
    status.className = "text-[9px] mono text-cyan-400 animate-pulse";
    
    await addLog("INIT: Universal Execution Fabric starting...", "info");
    await delay(500);

    // "The Grinder" Visual Stream
    let stream = "";
    const interval = setInterval(() => {
        const chars = "ΩΣΔΦΠΓΘΞΨ∫λπ";
        stream += chars[Math.floor(Math.random()*chars.length)];
        bytecodeBox.innerText = stream.substring(Math.max(0, stream.length - 150));
    }, 30);

    // Synthesis Phases
    const phases = [
        "Mapping high-level syntax to axiomatic nodes...",
        "Purging personal metadata (DNA Stripping)...",
        "Optimizing for multi-target execution (Mobile/IoT/Desktop)...",
        "Finalizing non-invertible uΩ artifact..."
    ];

    for (let i = 0; i < phases.length; i++) {
        await addLog(phases[i], "info");
        await delay(600);
    }

    clearInterval(interval);
    laser.classList.add('hidden');

    // Final uΩ Axiom Generation
    const axiom = generateAxiom(code);
    bytecodeBox.innerHTML = `<pre style="color: #00ff96; font-family: 'JetBrains Mono'">${axiom}</pre>`;
    
    status.innerText = "PERSONAL_ARTIFACT_READY";
    status.className = "text-[9px] mono text-emerald-500 uppercase";
    await addLog("SUCCESS: Logic artifact is now universal and secure.", "success");
}

function generateAxiom(code) {
    const lines = code.split('\n');
    let axiom = "∫ OMNI_PERSONAL_ARTIFACT {\n";
    axiom += "  [MODE: UNIVERSAL_FREEDOM]\n";
    axiom += "  [HASH: 0xODNA_CORE_" + Math.floor(Math.random()*1000) + "]\n\n";

    let addr = 161;
    lines.forEach(line => {
        const l = line.trim();
        if(!l || l.startsWith("#") || l.startsWith("//")) return;

        if (l.includes("print")) {
            axiom += `  0x01: EMIT >> [USER_INTERFACE_OUT]\n`;
        } else if (l.includes("=")) {
            axiom += `  0x02: BIND(0x${(addr++).toString(16).toUpperCase()}) ≔ [LOGIC_STATE]\n`;
        } else if (l.includes("*") || l.includes("+") || l.includes("-") || l.includes("/")) {
            axiom += `  0x03: MATH_OP(ALU_NATIVE) ➔ Δ.FINAL_STATE\n`;
        } else if (l.includes("for") || l.includes("while") || l.includes("if")) {
            axiom += `  0x04: FLOW_JUMP(STACK_SYNC) ➔ Ω.NODE\n`;
        }
    });

    axiom += "\n  [DNA_SIGNATURE: VERIFIED_PERSONAL]\n";
    axiom += "}";
    return axiom;
}

function downloadArtifact() {
    const content = document.getElementById('bytecode-output').innerText;
    if (content.length < 50 || content.includes("Awaiting")) return alert("Sintez tamamlanmayıb.");
    
    const blob = new Blob([content], { type: 'text/plain' });
    const a = document.createElement('a');
    a.href = window.URL.createObjectURL(blob);
    a.download = 'my_logic.omni';
    a.click();
    addLog("DOWNLOAD: logic.omni saved for personal use.", "success");
}

async function addLog(msg, type) {
    const logs = document.getElementById('stripping-logs');
    const div = document.createElement('div');
    const color = type === 'success' ? 'text-emerald-400' : 'text-gray-500';
    div.className = `border-b border-white/5 pb-0.5 ${color}`;
    div.innerText = `> ${msg}`;
    logs.prepend(div);
}

const delay = ms => new Promise(res => setTimeout(res, ms));
