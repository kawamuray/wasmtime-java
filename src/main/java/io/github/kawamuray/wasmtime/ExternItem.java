package io.github.kawamuray.wasmtime;

public class ExternItem {

	private final String name;

	private final Extern extern;

	public ExternItem(String name, Extern extern) {
		this.name = name;
		this.extern = extern;
	}

	public String getName() {
		return name;
	}

	public Extern getExtern() {
		return extern;
	}
	
}
