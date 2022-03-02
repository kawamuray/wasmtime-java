package io.github.kawamuray.wasmtime;

import lombok.Value;
import lombok.experimental.Accessors;

/**
 * {@link Extern} with its module and name.
 */
@Value
@Accessors(fluent = true)
public class ExternItem {
	/**
	 * The module defining this extern.
	 */
	String module;
	/**
	 * Name of this extern.
	 */
	String name;
	/**
	 * Extern itself.
	 */
	Extern extern;
}
