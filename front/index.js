const API_BASE = "http://localhost:3000";

const countEl = document.getElementById("count");
const statusEl = document.getElementById("status");
const incBtn = document.getElementById("incBtn");
const resetBtn = document.getElementById("resetBtn");

const setStatus = (message, isError = false) => {
	statusEl.textContent = message;
	statusEl.dataset.error = isError ? "true" : "false";
};

const updateCount = (value) => {
	countEl.textContent = String(value ?? 0);
};

const request = async (path, options = {}) => {
	const res = await fetch(`${API_BASE}${path}`, {
		headers: { "Content-Type": "application/json" },
		...options,
	});

	if (!res.ok) {
		const text = await res.text();
		throw new Error(text || `Request failed: ${res.status}`);
	}

	return res.json();
};

const loadCount = async () => {
	try {
		setStatus("Loading...");
		const data = await request("/count");
		updateCount(data.count);
		setStatus("Ready");
	} catch (err) {
		updateCount(0);
		setStatus(err.message || "Failed to load", true);
	}
};

const increment = async () => {
	try {
		incBtn.disabled = true;
		setStatus("Incrementing...");
		const data = await request("/inc", { method: "POST" });
		updateCount(data.count);
		setStatus("Ready");
	} catch (err) {
		setStatus(err.message || "Failed to increment", true);
	} finally {
		incBtn.disabled = false;
	}
};

const reset = async () => {
	try {
		resetBtn.disabled = true;
		setStatus("Resetting...");
		const data = await request("/reset", { method: "POST" });
		updateCount(data.count);
		setStatus("Ready");
	} catch (err) {
		setStatus(err.message || "Failed to reset", true);
	} finally {
		resetBtn.disabled = false;
	}
};

incBtn.addEventListener("click", increment);
resetBtn.addEventListener("click", reset);

loadCount();
