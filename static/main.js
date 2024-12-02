const $ = document.querySelector.bind(document);

const warnUrlInvalid = $("#warnUrlInvalid");
const warnUrlMissing = $("#warnUrlMissing");

const inputUrl = $('input[name="url"]');

const successDialog = $("#successDialog");
const successDialogUrl = $("#successDialogUrl");

$("#successDialogClose").addEventListener("click", function () {
	$("#successDialog").close();
});

$("#errorDialogClose").addEventListener("click", function () {
	$("#errorDialog").close();
});

inputUrl.addEventListener("keyup", function () {
	warnUrlInvalid.style.display = "none";
	warnUrlMissing.style.display = "none";
});

$("form").addEventListener("submit", function (event) {
	event.preventDefault();

	const url = inputUrl.value;

	if (!url) {
		warnUrlMissing.style.display = "block";
		return;
	}

	if (!isValidUrl(url)) {
		warnUrlInvalid.style.display = "block";
		return;
	}

	fetch(
		"/s",
		{
			method: 'POST',
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({ url })
		}
	).then(res => res.text()).then(short => {
		successDialogUrl.text = short;
		successDialogUrl.href = short;

		successDialog.showModal();
	}).catch(reason => {
		console.error(`Failed to shorten url: ${reason}`);
		$("#errorDialog").show();
	});
});

function isValidUrl(url) {
	try {
		new URL(url);
		return true;
	} catch {
		return false;
	}
}
